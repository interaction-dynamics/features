import { ChevronRight, Folder, FolderOpen, FoldVertical } from 'lucide-react'
import { useCallback, useEffect, useState } from 'react'

import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from '@/components/ui/collapsible'
import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
} from '@/components/ui/sidebar'
import { formatFeatureName } from '@/lib/format-feature-name'
import { cn } from '@/lib/utils'
import type { Feature } from '@/models/feature'
import { HelpButton } from './help-button'
import { OwnerDot } from './owner-dot'

interface NavFeaturesProps {
  items: Feature[]
  onFeatureClick?: (feature: Feature) => void
  activeFeature?: Feature | null
  collapseAll?: number
  onOpenStateChange?: (isOpen: boolean) => void
  isFirstLevel?: boolean
}

interface TreeNode {
  name: string
  path: string
  isFolder: boolean
  feature?: Feature
  children: Map<string, TreeNode>
  parentOwner?: string
}

function formatNodeName(name: string, isFeature: boolean) {
  return isFeature ? formatFeatureName(name) : name
}

// Find the common ancestor path of all features
function findCommonAncestor(paths: string[]): string {
  if (paths.length === 0) return ''
  if (paths.length === 1) {
    const parts = paths[0].split('/')
    return parts.slice(0, -1).join('/')
  }

  const splitPaths = paths.map((p) => p.split('/'))
  const minLength = Math.min(...splitPaths.map((p) => p.length))

  const commonParts: string[] = []
  for (let i = 0; i < minLength; i++) {
    const part = splitPaths[0][i]
    if (splitPaths.every((p) => p[i] === part)) {
      commonParts.push(part)
    } else {
      break
    }
  }

  return commonParts.join('/')
}

// Build a tree structure from features based on their paths
function buildPathTree(features: Feature[]): TreeNode {
  // Get all feature paths
  const allPaths: string[] = []

  function collectPaths(features: Feature[]) {
    for (const feature of features) {
      allPaths.push(feature.path)
      if (feature.features && feature.features.length > 0) {
        collectPaths(feature.features)
      }
    }
  }

  collectPaths(features)

  // Find common ancestor
  const commonAncestor = findCommonAncestor(allPaths)
  const ancestorParts = commonAncestor ? commonAncestor.split('/') : []

  // Create root node
  const root: TreeNode = {
    name:
      ancestorParts.length > 0 ? ancestorParts[ancestorParts.length - 1] : '/',
    path: commonAncestor || '/',
    isFolder: true,
    children: new Map(),
  }

  // Add all features to the tree
  function addFeatureToTree(
    feature: Feature,
    node: TreeNode,
    parentOwner?: string,
  ) {
    const relativePath = commonAncestor
      ? feature.path.slice(commonAncestor.length + 1)
      : feature.path

    if (!relativePath) return

    const parts = relativePath.split('/')
    let currentNode: TreeNode | undefined = node

    // Build the path up to the feature, creating folders as needed
    for (let i = 0; i < parts.length - 1; i++) {
      const part = parts[i]
      if (currentNode && !currentNode.children.has(part)) {
        const folderPath = commonAncestor
          ? `${commonAncestor}/${parts.slice(0, i + 1).join('/')}`
          : parts.slice(0, i + 1).join('/')

        currentNode.children.set(part, {
          name: part,
          path: folderPath,
          isFolder: true,
          children: new Map(),
          parentOwner,
        })
      }
      if (currentNode) {
        currentNode = currentNode.children.get(part)
      }
    }

    // Add the feature itself
    const featureName = feature.name
    const featureNode: TreeNode = {
      name: featureName,
      path: feature.path,
      isFolder: false,
      feature: feature,
      children: new Map(),
      parentOwner,
    }

    // Add nested features as children
    if (feature.features && feature.features.length > 0) {
      featureNode.isFolder = true
      for (const nestedFeature of feature.features) {
        const nestedNode: TreeNode = {
          name: nestedFeature.name,
          path: nestedFeature.path,
          isFolder:
            (nestedFeature.features && nestedFeature.features.length > 0) ??
            false,
          feature: nestedFeature,
          children: new Map(),
          parentOwner: feature.owner,
        }

        // Recursively add nested features
        if (nestedFeature.features && nestedFeature.features.length > 0) {
          function addNested(
            features: Feature[],
            parent: TreeNode,
            parentOwner: string,
          ) {
            for (const f of features) {
              const child: TreeNode = {
                name: f.name,
                path: f.path,
                isFolder: (f.features && f.features.length > 0) ?? false,
                feature: f,
                children: new Map(),
                parentOwner,
              }
              parent.children.set(f.name, child)
              if (f.features && f.features.length > 0) {
                addNested(f.features, child, f.owner)
              }
            }
          }
          addNested(nestedFeature.features, nestedNode, nestedFeature.owner)
        }

        featureNode.children.set(nestedFeature.name, nestedNode)
      }
    }
    if (currentNode) {
      currentNode.children.set(featureName, featureNode)
    }
  }

  // Process all top-level features
  for (const feature of features) {
    addFeatureToTree(feature, root)
  }

  // If root has only one child and it's a folder, skip the root
  if (root.children.size === 1) {
    const [firstChild] = root.children.values()
    if (firstChild.isFolder && !firstChild.feature) {
      return firstChild
    }
  }

  return root
}

function TreeNodeItem({
  node,
  onFeatureClick,
  activeFeature,
  depth = 0,
  collapseAll = 0,
  onOpenStateChange,
  isFirstLevel = false,
}: {
  node: TreeNode
  onFeatureClick?: (feature: Feature) => void
  activeFeature?: Feature | null
  depth?: number
  collapseAll?: number
  onOpenStateChange?: (isOpen: boolean) => void
  isFirstLevel?: boolean
}) {
  const hasChildren = node.children.size > 0
  const isActive = activeFeature?.path === node.path
  const isFeature = !!node.feature

  // Only show owner dot if the owner differs from parent
  const shouldShowOwnerDot =
    isFeature && node.feature && node.feature.owner !== node.parentOwner

  const [isOpen, setIsOpen] = useState(() => {
    // Auto-expand if this node is in the active path
    if (activeFeature) {
      return activeFeature.path.startsWith(node.path)
    }
    // Auto-expand folders up to features level
    return node.isFolder && !node.feature && node.name !== 'features'
  })

  // Close all folders when collapseAll changes
  useEffect(() => {
    if (collapseAll > 0) {
      setIsOpen(false)
    }
  }, [collapseAll])

  // Notify parent of open state changes (only for first level)
  useEffect(() => {
    if (hasChildren && onOpenStateChange && isFirstLevel) {
      onOpenStateChange(isOpen)
    }
  }, [isOpen, hasChildren, onOpenStateChange, isFirstLevel])

  // Skip rendering the 'features' folder itself if it has children
  if (node.name === 'features' && !isFeature && hasChildren) {
    return (
      <>
        {Array.from(node.children.values()).map((child) => (
          <TreeNodeItem
            key={child.path}
            node={child}
            onFeatureClick={onFeatureClick}
            activeFeature={activeFeature}
            depth={depth}
            collapseAll={collapseAll}
            onOpenStateChange={onOpenStateChange}
            isFirstLevel={isFirstLevel}
          />
        ))}
      </>
    )
  }

  if (!hasChildren) {
    return (
      <SidebarMenuItem>
        <SidebarMenuButton
          className={cn(
            'cursor-pointer w-full ',
            isFeature ? 'text-primary' : 'text-muted-foreground',
          )}
          asChild
          isActive={isActive}
          onClick={() => {
            console.log('click')
            if (node.feature) {
              onFeatureClick?.(node.feature)
            }
          }}
          tooltip={formatNodeName(node.name, isFeature)}
          title={formatNodeName(node.name, isFeature)}
        >
          <div className="flex items-center w-full gap-2">
            <span
              className="flex items-center gap-2 truncate cursor-pointer text-ellipsis flex-1"
              title={formatNodeName(node.name, isFeature)}
            >
              {!isFeature && <Folder className="h-4 w-4 opacity-60" />}
              {formatNodeName(node.name, isFeature)}
            </span>
            {shouldShowOwnerDot && <OwnerDot owner={node.feature.owner} />}
            {/* Reserve space for chevron to align with collapsible items */}
            <div className="w-4 shrink-0" />
          </div>
        </SidebarMenuButton>
      </SidebarMenuItem>
    )
  }

  return (
    <Collapsible
      key={node.path}
      asChild
      open={isOpen}
      onOpenChange={setIsOpen}
      className="group/collapsible"
    >
      <SidebarMenuItem>
        <CollapsibleTrigger asChild>
          <SidebarMenuButton
            className={cn(
              'cursor-pointer',
              isFeature ? 'text-primary' : 'text-muted-foreground',
            )}
            tooltip={formatNodeName(node.name, isFeature)}
            isActive={isActive}
            onClick={() => {
              if (node.feature) {
                onFeatureClick?.(node.feature)
              }
            }}
            title={formatNodeName(node.name, isFeature)}
          >
            <span
              className="flex items-center gap-2 truncate cursor-pointer flex-1"
              title={formatNodeName(node.name, isFeature)}
            >
              {!isFeature &&
                (isOpen ? (
                  <FolderOpen className="h-4 w-4 opacity-60" />
                ) : (
                  <Folder className="h-4 w-4 opacity-60" />
                ))}
              {formatNodeName(node.name, isFeature)}
            </span>
            {shouldShowOwnerDot && <OwnerDot owner={node.feature.owner} />}
            <ChevronRight
              className={cn(
                'ml-auto transition-transform duration-200',
                isOpen ? 'rotate-90' : '',
              )}
            />
          </SidebarMenuButton>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <SidebarMenuSub className="me-0 pe-0">
            {Array.from(node.children.values()).map((child) => (
              <TreeNodeItem
                key={child.path}
                node={child}
                onFeatureClick={onFeatureClick}
                activeFeature={activeFeature}
                depth={depth + 1}
                collapseAll={collapseAll}
                onOpenStateChange={onOpenStateChange}
                isFirstLevel={false}
              />
            ))}
          </SidebarMenuSub>
        </CollapsibleContent>
      </SidebarMenuItem>
    </Collapsible>
  )
}

function NavFeaturesMenu({
  items,
  onFeatureClick,
  activeFeature,
  collapseAll = 0,
  onOpenStateChange,
}: NavFeaturesProps) {
  if (items.length === 0) {
    return (
      <div className="px-2 py-4">
        <div className="flex flex-col items-start gap-2">
          <p className="text-sm text-muted-foreground">No features available</p>
          <HelpButton
            title="How to add a feature"
            url="https://github.com/interaction-dynamics/features/blob/master/FAQ.md#how-can-i-add-a-feature"
          />
        </div>
      </div>
    )
  }

  const tree = buildPathTree(items)

  const nodesToRender = Array.from(tree.children.values())

  return (
    <>
      {nodesToRender.map((node) => (
        <TreeNodeItem
          key={node.path}
          node={node}
          onFeatureClick={onFeatureClick}
          activeFeature={activeFeature}
          collapseAll={collapseAll}
          onOpenStateChange={onOpenStateChange}
          isFirstLevel={true}
        />
      ))}
    </>
  )
}

export function NavFeatures({
  items,
  onFeatureClick,
  activeFeature,
}: NavFeaturesProps) {
  const [collapseAll, setCollapseAll] = useState(0)
  const [openFoldersCount, setOpenFoldersCount] = useState(1)

  const handleCollapseAll = () => {
    setCollapseAll((prev) => prev + 1)
  }

  const handleOpenStateChange = useCallback((isOpen: boolean) => {
    setOpenFoldersCount((prev) => (isOpen ? prev + 1 : Math.max(0, prev - 1)))
  }, [])

  return (
    <SidebarGroup>
      <div className="flex items-center justify-between px-2 py-1.5">
        <SidebarGroupLabel className="flex-1">All Features</SidebarGroupLabel>
        {openFoldersCount > 0 && (
          <button
            type="button"
            onClick={handleCollapseAll}
            className="cursor-pointer inline-flex h-6 w-6 items-center justify-center rounded hover:bg-sidebar-accent hover:text-sidebar-accent-foreground text-sidebar-foreground/70 transition-colors"
            title="Collapse all folders"
            aria-label="Collapse all folders"
          >
            <FoldVertical className="h-3.5 w-3.5" />
          </button>
        )}
      </div>
      <SidebarMenu>
        <NavFeaturesMenu
          items={items}
          onFeatureClick={onFeatureClick}
          activeFeature={activeFeature}
          collapseAll={collapseAll}
          onOpenStateChange={handleOpenStateChange}
        />
      </SidebarMenu>
    </SidebarGroup>
  )
}
