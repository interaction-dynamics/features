import { Fragment } from 'react'
import { Link } from 'react-router'
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from '@/components/ui/breadcrumb'
import { formatFeatureName } from '@/lib/format-feature-name'
import type { Feature } from '@/models/feature'

interface FeatureBreadcrumbProps {
  feature: Feature | null
  onFeatureClick: (feature: Feature) => void
}

function buildBreadcrumbPath(feature: Feature | null): Feature[] {
  if (!feature) return []
  const path: Feature[] = []
  let current: Feature | undefined = feature
  while (current) {
    path.unshift(current)
    current = current.parent
  }
  return path
}

export function FeatureBreadcrumb({
  feature,
  onFeatureClick,
}: FeatureBreadcrumbProps) {
  const breadcrumbPath = buildBreadcrumbPath(feature)

  if (breadcrumbPath.length === 0) {
    return null
  }

  return (
    <Breadcrumb>
      <BreadcrumbList>
        {breadcrumbPath.map((item, index) => {
          const isLast = index === breadcrumbPath.length - 1
          return (
            <Fragment key={item.path}>
              <BreadcrumbItem className="hidden md:block">
                {isLast ? (
                  <BreadcrumbPage>
                    {formatFeatureName(item.name)}
                  </BreadcrumbPage>
                ) : (
                  <BreadcrumbLink
                    asChild
                    className="cursor-pointer"
                    onClick={() => onFeatureClick(item)}
                  >
                    <span>{formatFeatureName(item.name)}</span>
                  </BreadcrumbLink>
                )}
              </BreadcrumbItem>
              {!isLast && <BreadcrumbSeparator className="hidden md:block" />}
            </Fragment>
          )
        })}
      </BreadcrumbList>
    </Breadcrumb>
  )
}
