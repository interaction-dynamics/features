import { describe, expect, it } from 'vitest'
import { buildCommitUrl, buildPullRequestUrl } from '../git-utils'

// Test data definitions
const commitHash = 'abc123def456'
const shortHash = 'abc123'
const fullHash = 'abc123def456789012345678901234567890abcd'

// buildCommitUrl test cases
const buildCommitUrlCases = [
  // GitHub cases
  {
    description: 'GitHub SSH format',
    repositoryUrl: 'git@github.com:user/repo.git',
    commitHash,
    expected: 'https://github.com/user/repo/commit/abc123def456',
  },
  {
    description: 'GitHub HTTPS with .git',
    repositoryUrl: 'https://github.com/user/repo.git',
    commitHash,
    expected: 'https://github.com/user/repo/commit/abc123def456',
  },
  {
    description: 'GitHub HTTPS without .git',
    repositoryUrl: 'https://github.com/user/repo',
    commitHash,
    expected: 'https://github.com/user/repo/commit/abc123def456',
  },
  {
    description: 'GitHub URL with trailing slash',
    repositoryUrl: 'https://github.com/user/repo/',
    commitHash,
    expected: 'https://github.com/user/repo/commit/abc123def456',
  },
  {
    description: 'GitHub organization repos',
    repositoryUrl: 'git@github.com:organization/project.git',
    commitHash,
    expected: 'https://github.com/organization/project/commit/abc123def456',
  },
  {
    description: 'GitHub repos with hyphens and underscores',
    repositoryUrl: 'git@github.com:my-org/my_repo-name.git',
    commitHash,
    expected: 'https://github.com/my-org/my_repo-name/commit/abc123def456',
  },

  // GitLab cases
  {
    description: 'GitLab.com SSH format',
    repositoryUrl: 'git@gitlab.com:user/repo.git',
    commitHash,
    expected: 'https://gitlab.com/user/repo/-/commit/abc123def456',
  },
  {
    description: 'GitLab.com HTTPS format',
    repositoryUrl: 'https://gitlab.com/user/repo.git',
    commitHash,
    expected: 'https://gitlab.com/user/repo/-/commit/abc123def456',
  },
  {
    description: 'Self-hosted GitLab',
    repositoryUrl: 'https://gitlab.company.com/user/repo.git',
    commitHash,
    expected: 'https://gitlab.company.com/user/repo/-/commit/abc123def456',
  },
  {
    description: 'GitLab groups and subgroups',
    repositoryUrl: 'git@gitlab.com:group/subgroup/repo.git',
    commitHash,
    expected: 'https://gitlab.com/group/subgroup/repo/-/commit/abc123def456',
  },
  {
    description: 'GitLab nested group paths',
    repositoryUrl: 'https://gitlab.com/group/subgroup/team/repo.git',
    commitHash,
    expected:
      'https://gitlab.com/group/subgroup/team/repo/-/commit/abc123def456',
  },

  // Bitbucket cases
  {
    description: 'Bitbucket SSH format',
    repositoryUrl: 'git@bitbucket.org:user/repo.git',
    commitHash,
    expected: 'https://bitbucket.org/user/repo/commits/abc123def456',
  },
  {
    description: 'Bitbucket HTTPS format',
    repositoryUrl: 'https://bitbucket.org/user/repo.git',
    commitHash,
    expected: 'https://bitbucket.org/user/repo/commits/abc123def456',
  },
  {
    description: 'Bitbucket workspace repos',
    repositoryUrl: 'https://bitbucket.org/workspace/project.git',
    commitHash,
    expected: 'https://bitbucket.org/workspace/project/commits/abc123def456',
  },

  // Generic Git Hosting
  {
    description: 'Unknown git host (generic fallback)',
    repositoryUrl: 'https://git.example.com/user/repo.git',
    commitHash,
    expected: 'https://git.example.com/user/repo/commit/abc123def456',
  },
  {
    description: 'Self-hosted Gitea',
    repositoryUrl: 'https://gitea.example.com/user/repo.git',
    commitHash,
    expected: 'https://gitea.example.com/user/repo/commit/abc123def456',
  },
  {
    description: 'Custom git server',
    repositoryUrl: 'git@custom-git.company.com:team/project.git',
    commitHash,
    expected: 'https://custom-git.company.com/team/project/commit/abc123def456',
  },

  // Complex paths
  {
    description: 'Repository names with dots',
    repositoryUrl: 'https://github.com/user/repo.v2.git',
    commitHash,
    expected: 'https://github.com/user/repo.v2/commit/abc123def456',
  },
  {
    description: 'Repository names with numbers',
    repositoryUrl: 'git@github.com:user/repo123.git',
    commitHash,
    expected: 'https://github.com/user/repo123/commit/abc123def456',
  },

  // Different hash lengths
  {
    description: 'Short commit hash',
    repositoryUrl: 'https://github.com/user/repo.git',
    commitHash: shortHash,
    expected: 'https://github.com/user/repo/commit/abc123',
  },
  {
    description: 'Full 40-character commit hash',
    repositoryUrl: 'https://github.com/user/repo.git',
    commitHash: fullHash,
    expected: `https://github.com/user/repo/commit/${fullHash}`,
  },

  // Edge cases with whitespace
  {
    description: 'Repository URL with whitespace',
    repositoryUrl: '  https://github.com/user/repo.git  ',
    commitHash,
    expected: 'https://github.com/user/repo/commit/abc123def456',
  },

  // Multiple .git occurrences
  {
    description: 'Repository URL with multiple .git occurrences',
    repositoryUrl: 'https://github.com/user/repo.git.backup.git',
    commitHash,
    expected: 'https://github.com/user/repo.git.backup/commit/abc123def456',
  },

  // SSH URL with port (edge case)
  {
    description: 'SSH URL with port',
    repositoryUrl: 'ssh://git@github.com:22/user/repo.git',
    commitHash,
    expected: 'ssh://git@github.com:22/user/repo/commit/abc123def456',
  },
]

const buildCommitUrlNullCases = [
  {
    description: 'undefined repository URL',
    repositoryUrl: undefined,
    commitHash,
    expected: null,
  },
  {
    description: 'empty string repository URL',
    repositoryUrl: '',
    commitHash,
    expected: null,
  },
  {
    description: 'undefined commit hash',
    repositoryUrl: 'https://github.com/user/repo.git',
    commitHash: undefined,
    expected: null,
  },
  {
    description: 'empty commit hash',
    repositoryUrl: 'https://github.com/user/repo.git',
    commitHash: '',
    expected: null,
  },
  {
    description: 'both parameters empty',
    repositoryUrl: undefined,
    commitHash: '',
    expected: null,
  },
]

// buildPullRequestUrl test cases
const buildPullRequestUrlCases = [
  // GitHub cases
  {
    description: 'GitHub SSH format',
    repositoryUrl: 'git@github.com:user/repo.git',
    prNumber: 123,
    expected: 'https://github.com/user/repo/pull/123',
  },
  {
    description: 'GitHub HTTPS with .git',
    repositoryUrl: 'https://github.com/user/repo.git',
    prNumber: 456,
    expected: 'https://github.com/user/repo/pull/456',
  },
  {
    description: 'GitHub HTTPS without .git',
    repositoryUrl: 'https://github.com/user/repo',
    prNumber: 789,
    expected: 'https://github.com/user/repo/pull/789',
  },
  {
    description: 'GitHub URL with trailing slash',
    repositoryUrl: 'https://github.com/user/repo/',
    prNumber: 42,
    expected: 'https://github.com/user/repo/pull/42',
  },
  {
    description: 'GitHub organization repos',
    repositoryUrl: 'git@github.com:organization/project.git',
    prNumber: 99,
    expected: 'https://github.com/organization/project/pull/99',
  },
  {
    description: 'GitHub PR number as string',
    repositoryUrl: 'https://github.com/user/repo.git',
    prNumber: '555',
    expected: 'https://github.com/user/repo/pull/555',
  },

  // GitLab cases
  {
    description: 'GitLab.com SSH format',
    repositoryUrl: 'git@gitlab.com:user/repo.git',
    prNumber: 123,
    expected: 'https://gitlab.com/user/repo/-/merge_requests/123',
  },
  {
    description: 'GitLab.com HTTPS format',
    repositoryUrl: 'https://gitlab.com/user/repo.git',
    prNumber: 456,
    expected: 'https://gitlab.com/user/repo/-/merge_requests/456',
  },
  {
    description: 'Self-hosted GitLab',
    repositoryUrl: 'https://gitlab.company.com/user/repo.git',
    prNumber: 789,
    expected: 'https://gitlab.company.com/user/repo/-/merge_requests/789',
  },
  {
    description: 'GitLab groups and subgroups',
    repositoryUrl: 'git@gitlab.com:group/subgroup/repo.git',
    prNumber: 42,
    expected: 'https://gitlab.com/group/subgroup/repo/-/merge_requests/42',
  },
  {
    description: 'GitLab nested group paths',
    repositoryUrl: 'https://gitlab.com/group/subgroup/team/repo.git',
    prNumber: 99,
    expected: 'https://gitlab.com/group/subgroup/team/repo/-/merge_requests/99',
  },

  // Bitbucket cases
  {
    description: 'Bitbucket SSH format',
    repositoryUrl: 'git@bitbucket.org:user/repo.git',
    prNumber: 123,
    expected: 'https://bitbucket.org/user/repo/pull-requests/123',
  },
  {
    description: 'Bitbucket HTTPS format',
    repositoryUrl: 'https://bitbucket.org/user/repo.git',
    prNumber: 456,
    expected: 'https://bitbucket.org/user/repo/pull-requests/456',
  },
  {
    description: 'Bitbucket workspace repos',
    repositoryUrl: 'https://bitbucket.org/workspace/project.git',
    prNumber: 789,
    expected: 'https://bitbucket.org/workspace/project/pull-requests/789',
  },

  // Generic Git Hosting
  {
    description: 'Unknown git host (generic fallback)',
    repositoryUrl: 'https://git.example.com/user/repo.git',
    prNumber: 123,
    expected: 'https://git.example.com/user/repo/pull/123',
  },
  {
    description: 'Self-hosted Gitea',
    repositoryUrl: 'https://gitea.example.com/user/repo.git',
    prNumber: 456,
    expected: 'https://gitea.example.com/user/repo/pull/456',
  },
  {
    description: 'Custom git server',
    repositoryUrl: 'git@custom-git.company.com:team/project.git',
    prNumber: 789,
    expected: 'https://custom-git.company.com/team/project/pull/789',
  },

  // Repository URL with whitespace
  {
    description: 'Repository URL with whitespace',
    repositoryUrl: '  https://github.com/user/repo.git  ',
    prNumber: 42,
    expected: 'https://github.com/user/repo/pull/42',
  },

  // Large PR numbers
  {
    description: 'Large PR number',
    repositoryUrl: 'https://github.com/user/repo.git',
    prNumber: 9999,
    expected: 'https://github.com/user/repo/pull/9999',
  },
]

const buildPullRequestUrlNullCases = [
  {
    description: 'undefined repository URL',
    repositoryUrl: undefined,
    prNumber: 123,
    expected: null,
  },
  {
    description: 'empty string repository URL',
    repositoryUrl: '',
    prNumber: 123,
    expected: null,
  },
  {
    description: 'undefined PR number',
    repositoryUrl: 'https://github.com/user/repo.git',
    prNumber: undefined,
    expected: null,
  },
  {
    description: 'empty string PR number',
    repositoryUrl: 'https://github.com/user/repo.git',
    prNumber: '',
    expected: null,
  },
  {
    description: 'both parameters undefined',
    repositoryUrl: undefined,
    prNumber: undefined,
    expected: null,
  },
  {
    description: 'zero PR number',
    repositoryUrl: 'https://github.com/user/repo.git',
    prNumber: 0,
    expected: null,
  },
]

// Tests
describe('buildCommitUrl', () => {
  it.each(buildCommitUrlCases)(
    'should handle: $description',
    ({ repositoryUrl, commitHash, expected }) => {
      const result = buildCommitUrl(repositoryUrl, commitHash)
      expect(result).toBe(expected)
    },
  )

  it.each(buildCommitUrlNullCases)(
    'should return null for: $description',
    ({ repositoryUrl, commitHash, expected }) => {
      const result = buildCommitUrl(
        repositoryUrl as string | undefined,
        commitHash as string,
      )
      expect(result).toBe(expected)
    },
  )
})

describe('buildPullRequestUrl', () => {
  it.each(buildPullRequestUrlCases)(
    'should handle: $description',
    ({ repositoryUrl, prNumber, expected }) => {
      const result = buildPullRequestUrl(repositoryUrl, prNumber)
      expect(result).toBe(expected)
    },
  )

  it.each(buildPullRequestUrlNullCases)(
    'should return null for: $description',
    ({ repositoryUrl, prNumber, expected }) => {
      const result = buildPullRequestUrl(
        repositoryUrl as string | undefined,
        prNumber as number | string,
      )
      expect(result).toBe(expected)
    },
  )
})
