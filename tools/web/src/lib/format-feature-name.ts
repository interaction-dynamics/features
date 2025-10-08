export function formatFeatureName(name: string): string {
	return (
		name
			// Replace underscores and hyphens with spaces
			.replace(/[_-]/g, " ")
			// Insert space before uppercase letters (for camelCase and PascalCase)
			.replace(/([a-z])([A-Z])/g, "$1 $2")
			// Insert space before sequences of uppercase followed by lowercase (for acronyms)
			.replace(/([A-Z]+)([A-Z][a-z])/g, "$1 $2")
			// Capitalize first letter of each word
			.split(" ")
			.map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
			.join(" ")
			// Clean up multiple spaces
			.replace(/\s+/g, " ")
			.trim()
	);
}
