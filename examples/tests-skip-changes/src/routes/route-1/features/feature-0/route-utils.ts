// This is a utility file in routes/route-1/features/feature-0
// It should be different from libs/features/feature-0

export function handleRouteFeature0() {
	return "This is from routes/route-1/features/feature-0";
}

export const ROUTE_FEATURE_0_CONFIG = {
	name: "Route Feature 0",
	path: "/route-feature-0",
};

// Import from the libs feature-0 to test dependency resolution
import { createFeature0 } from "../../../../libs/features/feature-0/file";

export function useLibsFeature0() {
	return createFeature0();
}
