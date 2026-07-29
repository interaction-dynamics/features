export const getOwnerName = (owner: string | null | undefined): string => {
  return owner || 'Unknown'
}
