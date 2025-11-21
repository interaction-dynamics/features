export function createFeature4() {
  return {
    name: 'Feature 4',
    description: 'This is feature 4',
    enabled: true,
    version: '1.0.0',
    dependencies: ['feature-1', 'feature-2', 'feature-3'],
    init: (flag: boolean) => {
      if (flag) {
        console.log('Feature 4 2')
      }
      console.log('Feature 4 initialized')
    },
    destroy: () => {
      console.log('Feature 4 destroyed')
    },
  }
}
