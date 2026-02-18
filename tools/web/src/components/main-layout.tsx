import { useContext, useEffect, useState } from "react";
import {
	Outlet,
	useLocation,
	useNavigate,
	useSearchParams,
} from "react-router";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { FeatureSelectedContext } from "@/lib/feature-selected-context";
import { FeaturesContext } from "@/lib/features-context";
import type { Feature } from "@/models/feature";
import { AppSidebar } from "./app-sidebar";

export function MainLayout() {
	const [searchParams] = useSearchParams();

	const featurePath = searchParams.get("feature");

	const { features, featuresMap } = useContext(FeaturesContext);
	const [currentFeature, setCurrentFeature] = useState<Feature | null>(null);

	const navigate = useNavigate();

	useEffect(() => {
		if (features.length > 0) {
			if (featurePath) {
				// Find the updated version of the current feature by path using featuresMap
				const updatedFeature = featuresMap[featurePath];
				if (updatedFeature) {
					setCurrentFeature(updatedFeature);
				} else {
					// Feature was deleted, clear the selection
					setCurrentFeature(null);
				}
			} else {
				setCurrentFeature(features[0]);
			}
		} else {
			setCurrentFeature(null);
		}
	}, [features, featuresMap, featurePath]);

	const onSelectFeature = (feature: Feature) => {
		setCurrentFeature(feature);
		navigate({ pathname: "/", search: `?feature=${feature.path}` });
	};

	const location = useLocation();

	return (
		<FeatureSelectedContext.Provider
			value={{
				selectFeature: setCurrentFeature,
				selectedFeature: currentFeature,
			}}
		>
			<SidebarProvider>
				<AppSidebar
					activeFeature={location.pathname === "/" ? currentFeature : null}
					onFeatureClick={onSelectFeature}
				/>
				<SidebarInset>
					<div className="h-screen relative">
						<Outlet />
					</div>
				</SidebarInset>
			</SidebarProvider>
		</FeatureSelectedContext.Provider>
	);
}
