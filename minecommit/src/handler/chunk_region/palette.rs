use anyhow::{Context, Result};
use simdnbt::{
    Mutf8String,
    owned::{NbtCompound, NbtList, NbtTag},
};

type Cube<T, const SIZE: usize> = [[[T; SIZE]; SIZE]; SIZE];

/// A block state entry in a local section palette.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlockStateEntry {
    pub name: String,
    pub properties: Vec<(String, String)>,
}

#[inline]
fn fast_count_bits(n: usize) -> usize {
    let m = n.saturating_sub(1);
    usize::BITS as usize - m.leading_zeros() as usize
}

pub fn dump<T: Copy + Default + Eq, const SIZE: usize>(
    palette_rows: &[u64],
    palette_entries: &[T],
    count_bits: impl Fn(usize) -> usize,
) -> Box<Cube<T, SIZE>> {
    let bits = count_bits(palette_entries.len());
    let chunk_size = 64 / bits;
    let mask = (1u64 << bits) - 1;
    let mut cube = Box::new([[[T::default(); SIZE]; SIZE]; SIZE]);

    let flattened_cube = cube.as_flattened_mut().as_flattened_mut();
    for (cube_chunk, row) in flattened_cube.chunks_mut(chunk_size).zip(palette_rows) {
        let mut row = *row;
        for cube_elem in cube_chunk.iter_mut() {
            let palette_index = (row & mask) as usize;
            let palette_entry = palette_entries[palette_index];
            *cube_elem = palette_entry;
            row >>= bits;
        }
    }

    cube
}

/// Pack palette indices from a dense cube back into packed long array rows,
/// using the known palette size to determine bit width.
fn pack_from_cube<T: Into<u64> + Copy, const SIZE: usize>(
    cube: &Cube<T, SIZE>,
    palette_len: usize,
    count_bits: impl Fn(usize) -> usize,
) -> Vec<u64> {
    let bits = count_bits(palette_len);
    if bits == 0 {
        return vec![];
    }
    let chunk_size = 64 / bits;
    let mut rows = vec![0u64; (SIZE * SIZE * SIZE).div_ceil(chunk_size)];
    let flattened = cube.as_flattened().as_flattened();
    for (cube_chunk, row) in flattened.chunks(chunk_size).zip(rows.iter_mut()) {
        *row = 0;
        for &elem in cube_chunk.iter().rev() {
            *row <<= bits;
            *row |= elem.into();
        }
    }
    rows
}

/// Dump a biome palette section into `(palette, 64-byte indexed data)`.
/// Each byte in `data` is an index into the local `palette` vector.
pub fn dump_biome(nbt: &NbtCompound) -> Result<(Vec<String>, Box<[u8; 64]>)> {
    let palette: Vec<String> = nbt
        .list("palette")
        .with_context(|| format!("missing NBT list 'palette' in biome, got: {nbt:#?}"))?
        .strings()
        .with_context(|| format!("expect biome 'palette' is a NBT string list"))?
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let palette_indices: Vec<u8> = (0..palette.len() as u8).collect();
    let palette_rows = nbt.long_array("data");

    let cube: Box<Cube<u8, 4>> = if let Some(rows) = palette_rows {
        dump::<u8, 4>(bytemuck::cast_slice(rows), &palette_indices, fast_count_bits)
    } else {
        bytemuck::allocation::cast_box(Box::new([palette_indices.first().copied().unwrap_or(0); 64]))
    };

    Ok((palette, bytemuck::allocation::cast_box(cube)))
}

/// Load a biome palette section from `(palette, 64-byte indexed data)`.
pub fn load_biome(palette: Vec<String>, data: Box<[u8; 64]>) -> Result<NbtCompound> {
    let cube: Box<Cube<u8, 4>> = bytemuck::allocation::cast_box(data);

    let first = cube[0][0][0];
    let is_homo = cube.iter().flatten().flatten().all(|&x| x == first);

    let kvs = if is_homo {
        let entry = palette.get(first as usize).ok_or_else(|| {
            anyhow::anyhow!(
                "biome palette index {first} out of bounds (len={})",
                palette.len()
            )
        })?;
        vec![(
            "palette".into(),
            NbtTag::List(NbtList::from(vec![Mutf8String::from_string(
                entry.clone(),
            )])),
        )]
    } else {
        let rows = pack_from_cube(&cube, palette.len(), fast_count_bits);
        let entries: Vec<Mutf8String> = palette
            .into_iter()
            .map(|s| Mutf8String::from_string(s))
            .collect();
        vec![
            ("data".into(), NbtTag::LongArray(bytemuck::cast_vec(rows))),
            ("palette".into(), NbtTag::List(NbtList::from(entries))),
        ]
    };
    Ok(NbtCompound::from_values(kvs))
}

/// Dump a block state palette section into `(palette, 4096-u16 indexed data)`.
/// Each u16 in `data` is an index into the local `palette` vector.
pub fn dump_block(nbt: &NbtCompound) -> Result<(Vec<BlockStateEntry>, Box<[u16; 4096]>)> {
    let palette: Vec<BlockStateEntry> = nbt
        .list("palette")
        .with_context(|| format!("missing NBT list 'palette' in block_states, got: {nbt:#?}"))?
        .compounds()
        .with_context(|| format!("expect block 'palette' is a NBT compound list"))?
        .into_iter()
        .enumerate()
        .map(|(idx, entry)| {
            let name = entry
                .string("Name")
                .with_context(|| {
                    format!("missing NBT string 'palette.{idx}.Name', got: {entry:#?}")
                })?
                .to_string();
            let properties = if let Some(props) = entry.compound("Properties") {
                props
                    .iter()
                    .map(|(k, value)| {
                        let v = value.string().with_context(|| {
                            format!(
                                "expect 'palette.{idx}.Properties.{}' is a NBT string",
                                k.to_str()
                            )
                        })?;
                        Ok((k.to_string(), v.to_string()))
                    })
                    .collect::<Result<Vec<_>>>()?
            } else {
                Vec::new()
            };
            Ok(BlockStateEntry { name, properties })
        })
        .collect::<Result<Vec<_>>>()?;

    let palette_indices: Vec<u16> = (0..palette.len() as u16).collect();
    let palette_rows = nbt.long_array("data");

    let cube: Box<Cube<u16, 16>> = if let Some(rows) = palette_rows {
        let count_bits = |n: usize| fast_count_bits(n).max(4);
        dump::<u16, 16>(bytemuck::cast_slice(rows), &palette_indices, count_bits)
    } else {
        bytemuck::allocation::cast_box(Box::new([palette_indices.first().copied().unwrap_or(0); 4096]))
    };

    Ok((palette, bytemuck::allocation::cast_box(cube)))
}

/// Load a block state palette section from `(palette, 4096-u16 indexed data)`.
pub fn load_block(palette: Vec<BlockStateEntry>, data: Box<[u16; 4096]>) -> Result<NbtCompound> {
    let cube: Box<Cube<u16, 16>> = bytemuck::allocation::cast_box(data);
    let count_bits = |n: usize| fast_count_bits(n).max(4);
    let bits = count_bits(palette.len());
    let rows = pack_from_cube(&cube, palette.len(), count_bits);

    let entries: Vec<NbtTag> = palette
        .into_iter()
        .map(|entry| {
            let kvs: Vec<(Mutf8String, NbtTag)> = if entry.properties.is_empty() {
                vec![(
                    "Name".into(),
                    NbtTag::String(Mutf8String::from_string(entry.name)),
                )]
            } else {
                let props_kvs: Vec<(Mutf8String, NbtTag)> = entry
                    .properties
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            Mutf8String::from_string(k),
                            NbtTag::String(Mutf8String::from_string(v)),
                        )
                    })
                    .collect();
                vec![
                    (
                        "Name".into(),
                        NbtTag::String(Mutf8String::from_string(entry.name)),
                    ),
                    (
                        "Properties".into(),
                        NbtTag::Compound(NbtCompound::from_values(props_kvs)),
                    ),
                ]
            };
            NbtTag::Compound(NbtCompound::from_values(kvs))
        })
        .collect();

    let kvs = if rows.is_empty() || bits == 0 {
        vec![("palette".into(), NbtTag::List(NbtList::from(entries)))]
    } else {
        vec![
            ("data".into(), NbtTag::LongArray(bytemuck::cast_vec(rows))),
            ("palette".into(), NbtTag::List(NbtList::from(entries))),
        ]
    };
    Ok(NbtCompound::from_values(kvs))
}
