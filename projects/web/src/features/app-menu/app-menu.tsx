import {
  BookOpen,
  ChevronsUpDown,
  Globe,
  MessageSquareMore,
  SunMoon,
} from 'lucide-react'
import type * as React from 'react'
import { useTheme } from '@/components/theme-provider'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import {
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@/components/ui/sidebar'
import { useMetadata } from '@/hooks/use-metadata'
import {
  CHANGELOG,
  CHANGELOG_PREVIEW_SIZE,
  CHANGELOG_URL,
  DOCS_URL,
  FEEDBACK_URL,
  REPOSITORY_URL,
  WEBSITE_URL,
} from './changelog'
import { useChangelogSeen } from './use-changelog-seen'

const THEMES = [
  { value: 'light', label: 'Light' },
  { value: 'dark', label: 'Dark' },
  { value: 'system', label: 'System' },
] as const

/**
 * App menu at the bottom of the sidebar: website and help links, theme and the latest
 * changes ("What's new"), opened from the Features logo.
 */
export function AppMenu() {
  const { theme, setTheme } = useTheme()
  const { metadata } = useMetadata()
  const { hasUnread, markSeen } = useChangelogSeen()
  const version = metadata?.version

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <DropdownMenu
          onOpenChange={(open) => {
            // Mark as seen on close, so the unread hint stays while reading
            if (!open && hasUnread) markSeen()
          }}
        >
          <DropdownMenuTrigger asChild>
            <SidebarMenuButton
              size="lg"
              className="cursor-pointer data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
            >
              <span className="relative shrink-0">
                <img
                  src="./feature-icon.svg"
                  alt="Features logo"
                  className="size-8"
                />
                {hasUnread && (
                  <span
                    className="absolute -top-0.5 -right-0.5 size-2.5 rounded-full border-2 border-sidebar bg-primary"
                    title="New changes"
                  />
                )}
              </span>
              <span className="flex flex-col gap-0.5 leading-none min-w-0">
                <span className="font-medium">Features</span>
                {version && (
                  <span className="text-xs text-muted-foreground truncate">
                    v{version}
                  </span>
                )}
              </span>
              <ChevronsUpDown className="ml-auto size-4 text-muted-foreground" />
            </SidebarMenuButton>
          </DropdownMenuTrigger>
          <DropdownMenuContent
            side="top"
            align="start"
            sideOffset={8}
            className="w-(--radix-dropdown-menu-trigger-width) min-w-60 rounded-lg"
          >
            <DropdownMenuGroup>
              <ExternalItem href={WEBSITE_URL} icon={<Globe />}>
                Website
              </ExternalItem>
              <ExternalItem href={DOCS_URL} icon={<BookOpen />}>
                Docs
              </ExternalItem>
              <ExternalItem href={FEEDBACK_URL} icon={<MessageSquareMore />}>
                Give feedback
              </ExternalItem>
              <ExternalItem href={REPOSITORY_URL} icon={<GithubIcon />}>
                GitHub
              </ExternalItem>
              <DropdownMenuSub>
                <DropdownMenuSubTrigger>
                  <SunMoon />
                  Theme
                </DropdownMenuSubTrigger>
                <DropdownMenuSubContent>
                  <DropdownMenuRadioGroup
                    value={theme}
                    onValueChange={(value) =>
                      setTheme(value as (typeof THEMES)[number]['value'])
                    }
                  >
                    {THEMES.map(({ value, label }) => (
                      <DropdownMenuRadioItem key={value} value={value}>
                        {label}
                      </DropdownMenuRadioItem>
                    ))}
                  </DropdownMenuRadioGroup>
                </DropdownMenuSubContent>
              </DropdownMenuSub>
            </DropdownMenuGroup>
            <DropdownMenuSeparator />
            <DropdownMenuGroup>
              <DropdownMenuLabel className="flex items-center gap-2 text-xs font-normal text-muted-foreground">
                What&apos;s new
                {hasUnread && (
                  <span className="size-1.5 rounded-full bg-primary" />
                )}
              </DropdownMenuLabel>
              <div className="relative">
                {/* Dashed line linking the entries, through the dots */}
                <span
                  aria-hidden
                  className="pointer-events-none absolute top-4 bottom-4 left-4 border-l border-dashed border-muted-foreground/70"
                />
                {CHANGELOG.slice(0, CHANGELOG_PREVIEW_SIZE).map((entry) => (
                  <ExternalItem
                    key={entry.id}
                    href={entry.url}
                    icon={<TimelineDot />}
                  >
                    <span className="truncate">{entry.title}</span>
                  </ExternalItem>
                ))}
                <ExternalItem href={CHANGELOG_URL} icon={<TimelineDot />}>
                  Full changelog
                </ExternalItem>
              </div>
            </DropdownMenuGroup>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenuItem>
    </SidebarMenu>
  )
}

function ExternalItem({
  href,
  icon,
  children,
}: {
  href: string
  icon: React.ReactNode
  children: React.ReactNode
}) {
  return (
    <DropdownMenuItem asChild className="cursor-pointer">
      <a href={href} target="_blank" rel="noreferrer">
        {icon}
        {children}
      </a>
    </DropdownMenuItem>
  )
}

function TimelineDot() {
  return (
    <span className="flex size-4 shrink-0 items-center justify-center">
      <span className="size-1.5 rounded-full bg-muted-foreground" />
    </span>
  )
}

function GithubIcon() {
  return (
    <svg viewBox="0 0 98 96" aria-hidden>
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M48.854 0C21.839 0 0 22 0 49.217c0 21.756 13.993 40.172 33.405 46.69 2.427.49 3.316-1.059 3.316-2.362 0-1.141-.08-5.052-.08-9.127-13.59 2.934-16.42-5.867-16.42-5.867-2.184-5.704-5.42-7.17-5.42-7.17-4.448-3.015.324-3.015.324-3.015 4.934.326 7.523 5.052 7.523 5.052 4.367 7.496 11.404 5.378 14.235 4.074.404-3.178 1.699-5.378 3.074-6.6-10.839-1.141-22.243-5.378-22.243-24.283 0-5.378 1.94-9.778 5.014-13.2-.485-1.222-2.184-6.275.486-13.038 0 0 4.125-1.304 13.426 5.052a46.97 46.97 0 0 1 12.214-1.63c4.125 0 8.33.571 12.213 1.63 9.302-6.356 13.427-5.052 13.427-5.052 2.67 6.763.97 11.816.485 13.038 3.155 3.422 5.015 7.822 5.015 13.2 0 18.905-11.404 23.06-22.324 24.283 1.78 1.548 3.316 4.481 3.316 9.126 0 6.6-.08 11.897-.08 13.526 0 1.304.89 2.853 3.316 2.364 19.412-6.52 33.405-24.935 33.405-46.691C97.707 22 75.788 0 48.854 0z"
        fill="currentColor"
      />
    </svg>
  )
}
