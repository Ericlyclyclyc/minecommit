mod chunk_region;
mod entities_region;
mod gzip_nbt;
mod ignore;
mod poi_region;
mod raw;

use anyhow::{Context, Result};
use rayon::iter::ParallelIterator;
pub(crate) use chunk_region::ChunkRegionHandler;
pub(crate) use entities_region::EntitiesRegionHandler;
pub(crate) use gzip_nbt::GzipNbtHandler;
pub(crate) use ignore::IgnoreHandler;
pub(crate) use poi_region::PoiRegionHandler;
pub(crate) use raw::RawHandler;

use crate::odb::{OdbReader, OdbWriter};

pub(crate) trait Handler {
    /// Unique workspace name used as a prefix in storage so each handler's
    /// data lives under its own namespace.
    fn workspace(&self) -> &'static str;

    fn flatten(
        self,
        save_dir: &impl OdbReader,
        storage: &mut impl OdbWriter,
    ) -> Result<Vec<String>>;
    fn unflatten(
        self,
        save_dir: &mut impl OdbWriter,
        storage: &impl OdbReader,
    ) -> Result<Vec<String>>;
}

// ── Workspace-prefixed ODB wrappers ──────────────────────────────────────────

/// Transparently prepends `{prefix}/` to every key when reading from or
/// writing to a storage backend.
struct PrefixedWriter<'a, W: OdbWriter> {
    inner: &'a mut W,
    prefix: &'static str,
}

struct PrefixedReader<'a, R: OdbReader> {
    inner: &'a R,
    prefix: &'static str,
}

impl<W: OdbWriter> PrefixedWriter<'_, W> {
    fn prefixed(&self, key: &str) -> String {
        format!("{}/{}", self.prefix, key)
    }

    fn strip<'a>(&self, key: &'a str) -> Result<&'a str> {
        key.strip_prefix(&format!("{}/", self.prefix))
            .with_context(|| format!("storage key {key:?} missing workspace prefix {:?}", self.prefix))
    }
}

impl<R: OdbReader> PrefixedReader<'_, R> {
    fn prefixed(&self, key: &str) -> String {
        format!("{}/{}", self.prefix, key)
    }

    fn strip<'a>(&self, key: &'a str) -> Result<&'a str> {
        key.strip_prefix(&format!("{}/", self.prefix))
            .with_context(|| format!("storage key {key:?} missing workspace prefix {:?}", self.prefix))
    }
}

impl<W: OdbWriter> OdbReader for PrefixedWriter<'_, W> {
    fn get(&self, key: &str) -> Result<Vec<u8>> {
        self.inner.get(&self.prefixed(key))
    }

    fn get_par(&self, keys: &[&str]) -> Result<Vec<Vec<u8>>> {
        let prefixed: Vec<String> = keys.iter().map(|k| self.prefixed(k)).collect();
        let refs: Vec<&str> = prefixed.iter().map(|s| s.as_str()).collect();
        self.inner.get_par(&refs)
    }

    fn glob(&self, pattern: &str) -> Result<Vec<String>> {
        let prefixed_pattern = format!("{}/{}", self.prefix, pattern);
        self.inner
            .glob(&prefixed_pattern)?
            .into_iter()
            .map(|k| self.strip(&k).map(|s| s.to_string()))
            .collect()
    }
}

impl<W: OdbWriter> OdbWriter for PrefixedWriter<'_, W> {
    fn put(&mut self, key: &str, value: impl AsRef<[u8]>) -> Result<()> {
        self.inner.put(&self.prefixed(key), value)
    }

    fn put_par(
        &mut self,
        entries: impl rayon::iter::IntoParallelIterator<Item = (String, impl AsRef<[u8]>)>,
    ) -> Result<()> {
        let prefix = self.prefix;
        let prefixed: Vec<(String, Vec<u8>)> = entries
            .into_par_iter()
            .map(move |(key, value)| (format!("{}/{}", prefix, key), value.as_ref().to_vec()))
            .collect();
        self.inner.put_par(prefixed)
    }
}

impl<R: OdbReader> OdbReader for PrefixedReader<'_, R> {
    fn get(&self, key: &str) -> Result<Vec<u8>> {
        self.inner.get(&self.prefixed(key))
    }

    fn get_par(&self, keys: &[&str]) -> Result<Vec<Vec<u8>>> {
        let prefixed: Vec<String> = keys.iter().map(|k| self.prefixed(k)).collect();
        let refs: Vec<&str> = prefixed.iter().map(|s| s.as_str()).collect();
        self.inner.get_par(&refs)
    }

    fn glob(&self, pattern: &str) -> Result<Vec<String>> {
        let prefixed_pattern = format!("{}/{}", self.prefix, pattern);
        self.inner
            .glob(&prefixed_pattern)?
            .into_iter()
            .map(|k| self.strip(&k).map(|s| s.to_string()))
            .collect()
    }
}

// ── CrafterImpl ─────────────────────────────────────────────────────────────

pub(crate) enum CrafterImpl {
    Raw(RawHandler),
    GzipNbt(GzipNbtHandler),
    ChunkRegion(ChunkRegionHandler),
    EntitiesRegion(EntitiesRegionHandler),
    PoiRegion(PoiRegionHandler),
    Ignore(IgnoreHandler),
}

impl CrafterImpl {
    pub(crate) fn get_crafters(
        extra_patterns: Vec<String>,
        ignore_patterns: Vec<String>,
    ) -> Vec<Self> {
        vec![
            Self::ChunkRegion(ChunkRegionHandler {}),
            Self::EntitiesRegion(EntitiesRegionHandler {}),
            Self::PoiRegion(PoiRegionHandler {}),
            Self::Raw(RawHandler { extra_patterns }),
            Self::GzipNbt(GzipNbtHandler {}),
            Self::Ignore(IgnoreHandler { ignore_patterns }),
        ]
    }

}

impl Handler for CrafterImpl {
    fn workspace(&self) -> &'static str {
        match self {
            Self::Raw(h) => h.workspace(),
            Self::GzipNbt(h) => h.workspace(),
            Self::ChunkRegion(h) => h.workspace(),
            Self::EntitiesRegion(h) => h.workspace(),
            Self::PoiRegion(h) => h.workspace(),
            Self::Ignore(h) => h.workspace(),
        }
    }

    fn flatten(
        self,
        save_dir: &impl OdbReader,
        storage: &mut impl OdbWriter,
    ) -> Result<Vec<String>> {
        let mut prefixed = PrefixedWriter {
            inner: storage,
            prefix: self.workspace(),
        };
        match self {
            Self::Raw(c) => c.flatten(save_dir, &mut prefixed),
            Self::GzipNbt(c) => c.flatten(save_dir, &mut prefixed),
            Self::ChunkRegion(c) => c.flatten(save_dir, &mut prefixed),
            Self::EntitiesRegion(c) => c.flatten(save_dir, &mut prefixed),
            Self::PoiRegion(c) => c.flatten(save_dir, &mut prefixed),
            Self::Ignore(c) => c.flatten(save_dir, &mut prefixed),
        }
    }

    fn unflatten(
        self,
        save_dir: &mut impl OdbWriter,
        storage: &impl OdbReader,
    ) -> Result<Vec<String>> {
        let prefixed = PrefixedReader {
            inner: storage,
            prefix: self.workspace(),
        };
        match self {
            Self::Raw(c) => c.unflatten(save_dir, &prefixed),
            Self::GzipNbt(c) => c.unflatten(save_dir, &prefixed),
            Self::ChunkRegion(c) => c.unflatten(save_dir, &prefixed),
            Self::EntitiesRegion(c) => c.unflatten(save_dir, &prefixed),
            Self::PoiRegion(c) => c.unflatten(save_dir, &prefixed),
            Self::Ignore(c) => c.unflatten(save_dir, &prefixed),
        }
    }
}
