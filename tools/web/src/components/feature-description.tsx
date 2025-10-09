import ReactMarkdown from "react-markdown";

interface FeatureDescriptionProps {
	description: string;
}

export function FeatureDescription({ description }: FeatureDescriptionProps) {
	return (
		<div className="**:list-disc **:[ul]:ms-4 **:[p]:py-2 **:[h1]:scroll-m-20 **:[h1]:text-4xl **:[h1]:font-extrabold **:[h1]:tracking-tight **:[h1]:text-balance **:[h2]:scroll-m-20 **:[h2]:pb-2 **:[h2]:text-3xl **:[h2]:font-semibold **:[h2]:tracking-tight **:[h2]:first:mt-0 ">
			<ReactMarkdown>{description}</ReactMarkdown>
		 </div>
	);
}
