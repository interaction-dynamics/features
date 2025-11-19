import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from './ui/card'

interface StatsCardProps {
  title: string
  value: string | number
  description?: string
  subtitle?: string | number
}

export function StatsCard({
  title,
  value,
  description,
  subtitle,
}: StatsCardProps) {
  return (
    <Card className="@container/card flex flex-col justify-between">
      <CardHeader className="pb-3">
        <CardDescription>{title}</CardDescription>
        <CardTitle className="text-2xl font-semibold tabular-nums @[250px]/card:text-3xl">
          {value}
        </CardTitle>
      </CardHeader>
      <CardFooter className="flex-col items-start gap-1.5 text-sm">
        {subtitle && <div className="text-muted-foreground">{subtitle}</div>}
        {description && (
          <div className="text-muted-foreground">{description}</div>
        )}
      </CardFooter>
    </Card>
  )
}
