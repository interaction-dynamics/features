import { ChevronRight, Folder, FolderOpen } from 'lucide-react'
import { useState } from 'react'

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
import type { Feature } from '@/models/feature'
import { HelpButton } from './help-button'

interface NavFeaturesProps {
  items: Feature[]
  onFeatureClick?: (feature: Feature) => void
  activeFeature?: Feature | null
}

interface TreeNode {
  name: string
  path: string
  isFolder: boolean
  feature?: Feature
  children: Map<string, TreeNode>
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
  function addFeatureToTree(feature: Feature, node: TreeNode) {
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
        })
      }
      if (currentNode) {
        currentNode = currentNode.children.get(part)
      }
    }

    // Add the feature itself
    const featureName = parts[parts.length - 1]
    const featureNode: TreeNode = {
      name: featureName,
      path: feature.path,
      isFolder: false,
      feature: feature,
      children: new Map(),
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
        }

        // Recursively add nested features
        if (nestedFeature.features && nestedFeature.features.length > 0) {
          function addNested(features: Feature[], parent: TreeNode) {
            for (const f of features) {
              const child: TreeNode = {
                name: f.name,
                path: f.path,
                isFolder: (f.features && f.features.length > 0) ?? false,
                feature: f,
                children: new Map(),
              }
              parent.children.set(f.name, child)
              if (f.features && f.features.length > 0) {
                addNested(f.features, child)
              }
            }
          }
          addNested(nestedFeature.features, nestedNode)
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
}: {
  node: TreeNode
  onFeatureClick?: (feature: Feature) => void
  activeFeature?: Feature | null
  depth?: number
}) {
  const [isOpen, setIsOpen] = useState(() => {
    // Auto-expand if this node is in the active path
    if (activeFeature) {
      return activeFeature.path.startsWith(node.path)
    }
    // Auto-expand folders up to features level
    return node.isFolder && !node.feature && node.name !== 'features'
  })

  const hasChildren = node.children.size > 0
  const isActive = activeFeature?.path === node.path
  const isFeature = !!node.feature

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
          />
        ))}
      </>
    )
  }

  if (!hasChildren) {
    return (
      <SidebarMenuItem>
        <SidebarMenuButton
          className="cursor-pointer w-full"
          asChild
          isActive={isActive}
          onClick={() => node.feature && onFeatureClick?.(node.feature)}
        >
          <span className="flex items-center gap-2">
            {!isFeature && <Folder className="h-4 w-4 opacity-60" />}
            {isFeature ? formatFeatureName(node.name) : node.name}
          </span>
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
            className="cursor-pointer"
            tooltip={isFeature ? formatFeatureName(node.name) : node.name}
            isActive={isActive}
            onClick={() => {
              if (node.feature) {
                onFeatureClick?.(node.feature)
              }
              setIsOpen(!isOpen)
            }}
          >
            <span className="flex items-center gap-2">
              {!isFeature &&
                (isOpen ? (
                  <FolderOpen className="h-4 w-4 opacity-60" />
                ) : (
                  <Folder className="h-4 w-4 opacity-60" />
                ))}
              {isFeature ? formatFeatureName(node.name) : node.name}
            </span>
            <ChevronRight className="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
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
  return (
    <SidebarGroup>
      <SidebarGroupLabel>All Features</SidebarGroupLabel>
      <SidebarMenu>
        <NavFeaturesMenu
          items={items}
          onFeatureClick={onFeatureClick}
          activeFeature={activeFeature}
        />
      </SidebarMenu>
    </SidebarGroup>
  )
}
