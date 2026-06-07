import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@/components/ui/card"
import {
  Table,
  TableHeader,
  TableBody,
  TableHead,
  TableRow,
  TableCell,
} from "@/components/ui/table"
import {
  HoverCard,
  HoverCardTrigger,
  HoverCardContent,
} from "@/components/ui/hover-card"
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from "@/components/ui/empty"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Field, FieldGroup, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { useState } from "react"
import { Trash2, HardDrive } from "lucide-react"

interface Save {
  name: string
  path: string
  repoPath: string
  remoteRepoPath: string
}

function EmptySave({ onAddTrack }: { onAddTrack: () => void }) {
  return (
    <Empty>
      <EmptyHeader>
        <EmptyMedia variant="icon">
          <HardDrive />
        </EmptyMedia>
        <EmptyTitle>跟踪一个存档</EmptyTitle>
        <EmptyDescription>
          <p>MineCommit 还没有跟踪任何存档</p>
          <p>点击按钮来跟踪一个已有的存档</p>
        </EmptyDescription>
      </EmptyHeader>
      <EmptyContent>
        <Button onClick={onAddTrack}>添加跟踪</Button>
      </EmptyContent>
    </Empty>
  )
}

function AddTrackDialog({
  open,
  onOpenChange,
}: {
  open: boolean
  onOpenChange: (open: boolean) => void
}) {
  const [name, setName] = useState("")
  const [path, setPath] = useState("")
  const [localRepoPath, setLocalRepoPath] = useState("")
  const [remoteRepoPath, setRemoteRepoPath] = useState("")

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    // TODO: 添加跟踪逻辑
    onOpenChange(false)
    setName("")
    setPath("")
    setRemoteRepoPath("")
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <form onSubmit={handleSubmit}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>添加跟踪</DialogTitle>
            <DialogDescription>选择一个 Minecraft 存档文件夹</DialogDescription>
          </DialogHeader>
          <FieldGroup>
            <Field>
              <FieldLabel htmlFor="save-path">存档路径</FieldLabel>
              <Input
                id="save-path"
                placeholder="/home/user/.minecraft/saves/我的世界"
                value={path}
                onChange={(e) => setPath(e.target.value)}
                required
              />
            </Field>
            <Field>
              <FieldLabel htmlFor="save-name">存档名称</FieldLabel>
              <Input
                id="save-name"
                placeholder="我的世界"
                value={name}
                onChange={(e) => setName(e.target.value)}
                required
              />
            </Field>
            <Field>
              <FieldLabel htmlFor="local-repo-path">本地仓库路径</FieldLabel>
              <Input
                id="local-repo-path"
                placeholder="/home/user/.minecraft/minecommit/我的世界.git"
                value={localRepoPath}
                onChange={(e) => setLocalRepoPath(e.target.value)}
              />
            </Field>
            <Field>
              <FieldLabel htmlFor="remote-repo-path">
                远程仓库路径（可选）
              </FieldLabel>
              <Input
                id="remote-repo-path"
                placeholder="https://git.example.com/我的世界.git"
                value={remoteRepoPath}
                onChange={(e) => setRemoteRepoPath(e.target.value)}
              />
            </Field>
          </FieldGroup>
          <DialogFooter className="mt-6">
            <DialogClose render={<Button variant="outline" />}>
              取消
            </DialogClose>
            <Button type="submit">跟踪</Button>
          </DialogFooter>
        </DialogContent>
      </form>
    </Dialog>
  )
}

const saves: Save[] = [
  // {
  //   name: "世界1",
  //   path: "/home/user/.minecraft/saves/世界1",
  //   repoPath: "/home/user/.minecraft/saves/世界1/.git",
  //   remoteRepoPath: "https://github.com/user/mc-world1.git",
  // },
  // {
  //   name: "创造测试",
  //   path: "/home/user/.minecraft/saves/创造测试",
  //   repoPath: "/home/user/.minecraft/saves/创造测试/.git",
  //   remoteRepoPath: "https://github.com/user/mc-creative.git",
  // },
  // {
  //   name: "红石实验室",
  //   path: "/home/user/.minecraft/saves/红石实验室",
  //   repoPath: "/home/user/.minecraft/saves/红石实验室/.git",
  //   remoteRepoPath: "https://github.com/user/mc-redstone.git",
  // },
  // {
  //   name: "生存存档",
  //   path: "/home/user/.minecraft/saves/生存存档",
  //   repoPath: "/home/user/.minecraft/saves/生存存档/.git",
  //   remoteRepoPath: "",
  // },
]

export function SaveManagePage() {
  const [dialogOpen, setDialogOpen] = useState(false)

  return (
    <div className="flex w-full flex-col gap-4 p-4">
      <Card>
        <CardHeader>
          <div className="flex items-end justify-between">
            <div>
              <CardTitle>存档列表</CardTitle>
              <CardDescription>管理 MineCommit 对存档的跟踪</CardDescription>
            </div>
            {saves.length === 0 || (
              <Button onClick={() => setDialogOpen(true)}>添加跟踪</Button>
            )}
          </div>
        </CardHeader>
        <CardContent>
          {saves.length === 0 ? (
            <EmptySave onAddTrack={() => setDialogOpen(true)} />
          ) : (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead className="text-muted-foreground">
                    存档名称
                  </TableHead>
                  <TableHead className="text-muted-foreground">
                    存档路径
                  </TableHead>
                  <TableHead>
                    <span className="sr-only">操作</span>
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {saves.map((save) => (
                  <HoverCard key={save.name}>
                    <HoverCardTrigger render={<TableRow />}>
                      <TableCell className="">{save.name}</TableCell>
                      <TableCell>{save.path}</TableCell>
                      <TableCell className="text-right">
                        <Button
                          variant="ghost"
                          size="icon-sm"
                          className="cursor-pointer"
                          onClick={(e) => e.stopPropagation()}
                        >
                          <Trash2 />
                        </Button>
                      </TableCell>
                    </HoverCardTrigger>
                    <HoverCardContent align="start" className="w-auto">
                      <div className="flex flex-col gap-3">
                        <div>
                          <p className="text-xs text-muted-foreground">
                            存档名称
                          </p>
                          <p className="font-medium">{save.name}</p>
                        </div>
                        <div>
                          <p className="text-xs text-muted-foreground">
                            存档路径
                          </p>
                          <p className="font-mono text-xs break-all">
                            {save.path}
                          </p>
                        </div>
                        <div>
                          <p className="text-xs text-muted-foreground">
                            仓库路径
                          </p>
                          <p className="font-mono text-xs break-all">
                            {save.repoPath}
                          </p>
                        </div>
                        <div>
                          <p className="text-xs text-muted-foreground">
                            远程仓库路径
                          </p>
                          {save.remoteRepoPath ? (
                            <p className="font-mono text-xs break-all">
                              {save.remoteRepoPath}
                            </p>
                          ) : (
                            <p className="font-mono text-xs break-all text-muted-foreground">
                              {"（未设置）"}
                            </p>
                          )}
                        </div>
                      </div>
                    </HoverCardContent>
                  </HoverCard>
                ))}
              </TableBody>
            </Table>
          )}
        </CardContent>
      </Card>
      <AddTrackDialog open={dialogOpen} onOpenChange={setDialogOpen} />
    </div>
  )
}
