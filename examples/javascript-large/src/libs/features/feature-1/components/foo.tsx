/** --feature-flag feature:feature-1, type: experiment, owner: #owner, introduced_on: 2025-12-31 */
// Import from feature-0 to create a dependency loop (feature-0 also imports from feature-1)
import { createFeature0 } from "../../feature-0/file";

export function Foo() {
	// This creates a dependency loop with feature-0
	const feature0 = createFeature0();
	return "foo";
}

// Export types for tight dependency demonstration
export type SomeType1 = { id: string };
export type SomeType2 = { name: string };
export type SomeType3 = { value: number };
export type SomeType4 = { enabled: boolean };
export type SomeType5 = { data: unknown };
export type SomeType6 = { config: Record<string, unknown> };
