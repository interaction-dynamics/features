// Multiple imports from feature-1 to demonstrate tight dependency (>5 from same file)
import { Foo } from '../feature-1/components/foo';
import type { SomeType1 } from '../feature-1/components/foo';
import type { SomeType2 } from '../feature-1/components/foo';
import type { SomeType3 } from '../feature-1/components/foo';
import type { SomeType4 } from '../feature-1/components/foo';
import type { SomeType5 } from '../feature-1/components/foo';
import type { SomeType6 } from '../feature-1/components/foo';

export {Bar} from './components/bar';

// Re-export Foo from feature-1
export { Foo };
