// Main export file for routes/route-1/features/feature-0

export {
	handleRouteFeature0,
	ROUTE_FEATURE_0_CONFIG,
	useLibsFeature0,
} from "./route-utils";

// Also import from Feature 1 to create more dependencies
import { Foo } from "../../../../libs/features/feature-1/components/foo";

export function withFoo() {
	return new Foo();
}
