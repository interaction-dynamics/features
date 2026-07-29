/**
 * Generate a consistent color from a string (owner name) using a hash function
 */
export function getOwnerColor(owner: string): string {
  if (!owner || owner.trim() === '') {
    return 'rgb(156, 163, 175)' // gray-400
  }

  // Simple hash function
  let hash = 0
  for (let i = 0; i < owner.length; i++) {
    hash = owner.charCodeAt(i) + ((hash << 5) - hash)
    hash = hash & hash // Convert to 32bit integer
  }

  // Generate HSL color with good saturation and lightness for visibility
  const hue = Math.abs(hash) % 360
  const saturation = 65 + (Math.abs(hash) % 20) // 65-85%
  const lightness = 50 + (Math.abs(hash) % 15) // 50-65%

  return `hsl(${hue}, ${saturation}%, ${lightness}%)`
}
