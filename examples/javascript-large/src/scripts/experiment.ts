// --feature-experiment feature:feature-1, hypothesis: increase-conversions, start_date: 2025-01-01

export class ExperimentTracker {
  private experimentId: string;
  
  constructor(id: string) {
    this.experimentId = id;
  }
  
  track(event: string): void {
    console.log(`Tracking ${event} for experiment ${this.experimentId}`);
  }
}
