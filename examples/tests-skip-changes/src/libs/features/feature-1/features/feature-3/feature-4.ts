export function createFeature3() {
  return {
    name: 'Feature 3',
    description: 'This is feature 3',
    enabled: true,
    version: '1.0.0',
    dependencies: ['feature-1', 'feature-2', 'feature-3'],
    init: (flag: boolean) => {
      if (flag) {
        console.log('Feature 3 2')
      }
      console.log('Feature 3 initialized')
    },
    destroy: () => {
      console.log('Feature 3 destroyed')
    },
  }
}
