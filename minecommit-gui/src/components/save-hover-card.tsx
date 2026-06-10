import {
  HoverCard,
  HoverCardTrigger,
  HoverCardContent,
} from "@/components/ui/hover-card"
import type { Save } from "@/contexts/saves"
import type { ReactElement } from "react"

interface SaveHoverCardProps {
  save: Save
  children: ReactElement
}

export function SaveHoverCard({ save, children }: SaveHoverCardProps) {
  return (
    <HoverCard>
      <HoverCardTrigger render={children}></HoverCardTrigger>
      <HoverCardContent align="start" className="w-auto text-xs">
        <div className="flex flex-col gap-2">
          <div>
            <p className="text-muted-foreground">存档名称</p>
            <p className="break-all">{save.name}</p>
          </div>
          <div>
            <p className="text-muted-foreground">最近访问</p>
            <p className="break-all">{save.last_access}</p>
          </div>
          <div>
            <p className="text-muted-foreground">存档路径</p>
            <p className="break-all">{save.path}</p>
          </div>
          <div>
            <p className="text-muted-foreground">仓库路径</p>
            <p className="break-all">{save.repo_path}</p>
          </div>
          <div>
            <p className="text-muted-foreground">远程仓库路径</p>
            {save.remote_repo_path ? (
              <p className="break-all">{save.remote_repo_path}</p>
            ) : (
              <p className="break-all text-muted-foreground">{"（未设置）"}</p>
            )}
          </div>
          <div>
            <p className="text-muted-foreground">默认分支</p>
            <p className="break-all">{save.default_branch || "—"}</p>
          </div>
        </div>
      </HoverCardContent>
    </HoverCard>
  )
}
