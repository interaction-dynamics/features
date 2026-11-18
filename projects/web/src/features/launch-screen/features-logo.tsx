import { useId } from 'react'
import { cn } from '@/lib/utils'

interface FeaturesLogoProps {
  className?: string
}

/**
 * The `{F}` logo (same drawing as `public/feature-icon.svg`), inlined so the
 * braces and the F can be animated separately.
 */
export function FeaturesLogo({ className }: FeaturesLogoProps) {
  const id = useId()
  const gradientId = `${id}-gradient`
  const gradientFId = `${id}-gradient-f`

  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 64 64"
      fill="none"
      aria-hidden="true"
      className={cn('block', className)}
    >
      <defs>
        <linearGradient
          id={gradientId}
          x1="32"
          y1="14"
          x2="32"
          y2="50"
          gradientUnits="userSpaceOnUse"
        >
          <stop stopColor="#F9F9F9" />
          <stop offset="1" stopColor="#B3B3B3" />
        </linearGradient>
        <linearGradient
          id={gradientFId}
          x1="32"
          y1="-10.86"
          x2="32"
          y2="74.86"
          gradientUnits="userSpaceOnUse"
        >
          <stop stopColor="#F9F9F9" />
          <stop offset="1" stopColor="#B3B3B3" />
        </linearGradient>
      </defs>
      <rect width="64" height="64" rx="16" fill="#1A1A1D" />
      <rect
        x="0.5"
        y="0.5"
        width="63"
        height="63"
        rx="15.5"
        stroke="#FFFFFF"
        strokeOpacity="0.08"
      />
      <g
        stroke={`url(#${gradientId})`}
        strokeWidth="4"
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        <path
          data-logo-part="brace"
          pathLength={1}
          d="M20 16c-4 0-5 2-5 5v6c0 3-2 5-4 5 2 0 4 2 4 5v6c0 3 1 5 5 5"
        />
        <path
          data-logo-part="brace"
          pathLength={1}
          d="M44 16c4 0 5 2 5 5v6c0 3 2 5 4 5-2 0-4 2-4 5v6c0 3-1 5-5 5"
        />
      </g>
      <g data-logo-part="letter">
        <path
          fill={`url(#${gradientFId})`}
          transform="translate(32 32) scale(0.42) translate(-32 -32)"
          d="M21 14h22a5 5 0 0 1 0 10H26v4h12a5 5 0 0 1 0 10H26v7a5 5 0 0 1-10 0V19a5 5 0 0 1 5-5Z"
        />
      </g>
    </svg>
  )
}
