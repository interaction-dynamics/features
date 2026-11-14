import { DataBuildDate } from './data-build-date'
import { Separator } from './ui/separator'
import { SidebarTrigger } from './ui/sidebar'

type HeaderProps = {
  children: React.ReactNode
}

export function Header({ children }: HeaderProps) {
  return (
    <header className="sticky top-0 bg-background z-50 flex h-16 shrink-0 items-center gap-2 border-b">
      <div className="flex items-center gap-2 px-3">
        <SidebarTrigger />
        <Separator orientation="vertical" className="mr-2 h-4" />
        {children}
      </div>
      <div className="ml-auto px-3 flex items-center gap-2">
        <DataBuildDate />
      </div>
    </header>
  )
}
