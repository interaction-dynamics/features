/**
 * Utility functions for working with git repositories
 */

/**
 * Converts a git repository URL to a web-accessible commit URL
 * Supports GitHub, GitLab, Bitbucket, and other common git hosting platforms
 */
export function buildCommitUrl(
  repositoryUrl: string | undefined,
  commitHash: string,
): string | null {
  if (!repositoryUrl || !commitHash) {
    return null
  }

  // Normalize the repository URL
  let normalizedUrl = repositoryUrl.trim()

  // Convert SSH URLs to HTTPS format
  // git@github.com:user/repo.git -> https://github.com/user/repo
  const sshPattern = /^git@([^:]+):(.+?)(\.git)?$/
  const sshMatch = normalizedUrl.match(sshPattern)
  if (sshMatch) {
    const [, host, path] = sshMatch
    normalizedUrl = `https://${host}/${path}`
  }

  // Remove trailing .git if present
  normalizedUrl = normalizedUrl.replace(/\.git$/, '')

  // Remove trailing slash
  normalizedUrl = normalizedUrl.replace(/\/$/, '')

  // Determine the platform and build the appropriate commit URL
  if (normalizedUrl.includes('github.com')) {
    return `${normalizedUrl}/commit/${commitHash}`
  }

  if (
    normalizedUrl.includes('gitlab.com') ||
    normalizedUrl.includes('gitlab')
  ) {
    return `${normalizedUrl}/-/commit/${commitHash}`
  }

  if (normalizedUrl.includes('bitbucket.org')) {
    return `${normalizedUrl}/commits/${commitHash}`
  }

  // Generic fallback for other git hosting platforms
  // Most follow the GitHub pattern
  return `${normalizedUrl}/commit/${commitHash}`
}

/**
 * Converts a git repository URL to a web-accessible pull request URL
 * Supports GitHub, GitLab, Bitbucket, and other common git hosting platforms
 */
export function buildPullRequestUrl(
  repositoryUrl: string | undefined,
  prNumber: number | string,
): string | null {
  if (!repositoryUrl || !prNumber) {
    return null
  }

  // Normalize the repository URL
  let normalizedUrl = repositoryUrl.trim()

  // Convert SSH URLs to HTTPS format
  const sshPattern = /^git@([^:]+):(.+?)(\.git)?$/
  const sshMatch = normalizedUrl.match(sshPattern)
  if (sshMatch) {
    const [, host, path] = sshMatch
    normalizedUrl = `https://${host}/${path}`
  }

  // Remove trailing .git if present
  normalizedUrl = normalizedUrl.replace(/\.git$/, '')

  // Remove trailing slash
  normalizedUrl = normalizedUrl.replace(/\/$/, '')

  // Determine the platform and build the appropriate PR URL
  if (normalizedUrl.includes('github.com')) {
    return `${normalizedUrl}/pull/${prNumber}`
  }

  if (
    normalizedUrl.includes('gitlab.com') ||
    normalizedUrl.includes('gitlab')
  ) {
    return `${normalizedUrl}/-/merge_requests/${prNumber}`
  }

  if (normalizedUrl.includes('bitbucket.org')) {
    return `${normalizedUrl}/pull-requests/${prNumber}`
  }

  // Generic fallback - most platforms follow GitHub pattern
  return `${normalizedUrl}/pull/${prNumber}`
}
