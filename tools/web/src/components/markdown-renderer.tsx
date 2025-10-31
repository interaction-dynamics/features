import type { Components } from 'react-markdown'
import ReactMarkdown from 'react-markdown'

interface MarkdownRendererProps {
  markdown: string
  className?: string
}

const components: Components = {
  // Headings
  h1: ({ children, ...props }) => (
    <h1
      className="scroll-m-20 text-4xl font-extrabold tracking-tight text-balance mt-6 lg:text-5xl"
      {...props}
    >
      {children}
    </h1>
  ),
  h2: ({ children, ...props }) => (
    <h2
      className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight mt-5 first:mt-0"
      {...props}
    >
      {children}
    </h2>
  ),
  h3: ({ children, ...props }) => (
    <h3
      className="scroll-m-20 text-2xl font-semibold tracking-tight mt-4"
      {...props}
    >
      {children}
    </h3>
  ),
  h4: ({ children, ...props }) => (
    <h4
      className="scroll-m-20 text-xl font-semibold tracking-tight mt-3"
      {...props}
    >
      {children}
    </h4>
  ),
  h5: ({ children, ...props }) => (
    <h5
      className="scroll-m-20 text-lg font-semibold tracking-tight mt-2"
      {...props}
    >
      {children}
    </h5>
  ),
  h6: ({ children, ...props }) => (
    <h6
      className="scroll-m-20 text-base font-semibold tracking-tight mt-2"
      {...props}
    >
      {children}
    </h6>
  ),

  // Paragraph
  p: ({ children, ...props }) => (
    <p className="leading-7 [&:not(:first-child)]:mt-6" {...props}>
      {children}
    </p>
  ),

  // Links
  a: ({ children, href, ...props }) => (
    <a
      href={href}
      className="font-medium text-primary underline underline-offset-4 hover:text-primary/80"
      target={href?.startsWith('http') ? '_blank' : undefined}
      rel={href?.startsWith('http') ? 'noopener noreferrer' : undefined}
      {...props}
    >
      {children}
    </a>
  ),

  // Blockquote
  blockquote: ({ children, ...props }) => (
    <blockquote
      className="mt-6 border-l-2 border-muted-foreground/20 pl-6 italic text-muted-foreground"
      {...props}
    >
      {children}
    </blockquote>
  ),

  // Lists
  ul: ({ children, ...props }) => (
    <ul className="my-6 ml-6 list-disc [&>li]:mt-2" {...props}>
      {children}
    </ul>
  ),
  ol: ({ children, ...props }) => (
    <ol className="my-6 ml-6 list-decimal [&>li]:mt-2" {...props}>
      {children}
    </ol>
  ),
  li: ({ children, ...props }) => (
    <li className="leading-7" {...props}>
      {children}
    </li>
  ),

  // Code
  code: ({ children, className, ...props }) => {
    const isInline = !className?.includes('language-')

    if (isInline) {
      return (
        <code
          className="relative rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold"
          {...props}
        >
          {children}
        </code>
      )
    }

    // For code blocks
    return (
      <code
        className="relative block w-full overflow-x-auto rounded-lg bg-muted p-4 font-mono text-sm"
        {...props}
      >
        {children}
      </code>
    )
  },
  pre: ({ children, ...props }) => (
    <pre
      className="mb-4 mt-6 overflow-x-auto rounded-lg border bg-muted p-4"
      {...props}
    >
      {children}
    </pre>
  ),

  // Table
  table: ({ children, ...props }) => (
    <div className="my-6 w-full overflow-y-auto">
      <table className="w-full" {...props}>
        {children}
      </table>
    </div>
  ),
  thead: ({ children, ...props }) => (
    <thead className="border-b" {...props}>
      {children}
    </thead>
  ),
  tbody: ({ children, ...props }) => <tbody {...props}>{children}</tbody>,
  tr: ({ children, ...props }) => (
    <tr className="m-0 border-t p-0 even:bg-muted" {...props}>
      {children}
    </tr>
  ),
  th: ({ children, ...props }) => (
    <th
      className="border px-4 py-2 text-left font-bold [&[align=center]]:text-center [&[align=right]]:text-right"
      {...props}
    >
      {children}
    </th>
  ),
  td: ({ children, ...props }) => (
    <td
      className="border px-4 py-2 text-left [&[align=center]]:text-center [&[align=right]]:text-right"
      {...props}
    >
      {children}
    </td>
  ),

  // Horizontal Rule
  hr: ({ ...props }) => (
    <hr className="my-8 border-t border-muted-foreground/20" {...props} />
  ),

  // Strong/Bold
  strong: ({ children, ...props }) => (
    <strong className="font-semibold" {...props}>
      {children}
    </strong>
  ),

  // Emphasis/Italic
  em: ({ children, ...props }) => (
    <em className="italic" {...props}>
      {children}
    </em>
  ),

  // Images
  img: ({ src, alt, ...props }) => (
    <img
      src={src}
      alt={alt}
      className="my-6 rounded-lg border"
      loading="lazy"
      {...props}
    />
  ),

  // Definition List (less common but good to have)
  dl: ({ children, ...props }) => (
    <dl className="my-6 space-y-2" {...props}>
      {children}
    </dl>
  ),
  dt: ({ children, ...props }) => (
    <dt className="font-semibold leading-7" {...props}>
      {children}
    </dt>
  ),
  dd: ({ children, ...props }) => (
    <dd className="ml-6 leading-7 text-muted-foreground" {...props}>
      {children}
    </dd>
  ),

  // Mark/Highlight
  mark: ({ children, ...props }) => (
    <mark className="bg-yellow-200 dark:bg-yellow-900 px-1 rounded" {...props}>
      {children}
    </mark>
  ),

  // Subscript and Superscript
  sub: ({ children, ...props }) => (
    <sub className="text-xs" {...props}>
      {children}
    </sub>
  ),
  sup: ({ children, ...props }) => (
    <sup className="text-xs" {...props}>
      {children}
    </sup>
  ),

  // Keyboard input
  kbd: ({ children, ...props }) => (
    <kbd
      className="pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground opacity-100"
      {...props}
    >
      {children}
    </kbd>
  ),

  // Delete (strikethrough)
  del: ({ children, ...props }) => (
    <del className="line-through opacity-70" {...props}>
      {children}
    </del>
  ),

  // Abbreviation
  abbr: ({ children, title, ...props }) => (
    <abbr
      title={title}
      className="cursor-help underline decoration-dotted underline-offset-4"
      {...props}
    >
      {children}
    </abbr>
  ),
}

export function MarkdownRenderer({
  markdown,
  className = '',
}: MarkdownRendererProps) {
  return (
    <div
      className={`prose prose-neutral dark:prose-invert max-w-none ${className}`}
    >
      <ReactMarkdown components={components}>{markdown}</ReactMarkdown>
    </div>
  )
}
