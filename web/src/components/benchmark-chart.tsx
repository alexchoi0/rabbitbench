"use client";

import { useState, useMemo } from "react";
import {
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
  ComposedChart,
} from "recharts";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";

export interface Branch {
  id: string;
  name: string;
}

export interface Testbed {
  id: string;
  name: string;
}

export interface Measure {
  id: string;
  name: string;
  units: string | null;
}

export interface Benchmark {
  id: string;
  name: string;
}

export interface DataPoint {
  x: string;
  y: number;
  lower: number | null;
  upper: number | null;
  gitHash: string | null;
}

export interface Series {
  benchmark: { id: string; name: string };
  branch: { id: string; name: string };
  testbed: { id: string; name: string };
  measure: { id: string; name: string; units: string | null };
  data: DataPoint[];
}

interface BenchmarkChartClientProps {
  benchmarks: Benchmark[];
  branches: Branch[];
  testbeds: Testbed[];
  measures: Measure[];
  initialSeries: Series[];
}

const COLORS = [
  "hsl(221, 83%, 53%)",
  "hsl(142, 76%, 36%)",
  "hsl(38, 92%, 50%)",
  "hsl(0, 84%, 60%)",
  "hsl(262, 83%, 58%)",
  "hsl(199, 89%, 48%)",
];

export function BenchmarkChartClient({
  benchmarks,
  branches,
  testbeds,
  measures,
  initialSeries,
}: BenchmarkChartClientProps) {
  const [selectedBenchmarks, setSelectedBenchmarks] = useState<string[]>(
    benchmarks.slice(0, 1).map((b) => b.id)
  );
  const [selectedBranches, setSelectedBranches] = useState<string[]>(
    branches.slice(0, 1).map((b) => b.id)
  );
  const [selectedTestbeds, setSelectedTestbeds] = useState<string[]>(
    testbeds.slice(0, 1).map((t) => t.id)
  );
  const [selectedMeasures, setSelectedMeasures] = useState<string[]>(
    measures.slice(0, 1).map((m) => m.id)
  );

  const filteredSeries = useMemo(() => {
    return initialSeries.filter(
      (s) =>
        selectedBenchmarks.includes(s.benchmark.id) &&
        selectedBranches.includes(s.branch.id) &&
        selectedTestbeds.includes(s.testbed.id) &&
        selectedMeasures.includes(s.measure.id)
    );
  }, [initialSeries, selectedBenchmarks, selectedBranches, selectedTestbeds, selectedMeasures]);

  const chartData = useMemo(() => {
    return filteredSeries.length > 0 ? transformData(filteredSeries) : [];
  }, [filteredSeries]);

  const toggleSelection = (
    id: string,
    selected: string[],
    setSelected: (ids: string[]) => void
  ) => {
    if (selected.includes(id)) {
      if (selected.length > 1) {
        setSelected(selected.filter((s) => s !== id));
      }
    } else {
      setSelected([...selected, id]);
    }
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Performance Over Time</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
          <div>
            <label className="text-sm font-medium mb-2 block">Benchmarks</label>
            <div className="flex flex-wrap gap-1">
              {benchmarks.map((b) => (
                <Button
                  key={b.id}
                  variant={selectedBenchmarks.includes(b.id) ? "default" : "outline"}
                  size="sm"
                  onClick={() =>
                    toggleSelection(b.id, selectedBenchmarks, setSelectedBenchmarks)
                  }
                >
                  {b.name}
                </Button>
              ))}
            </div>
          </div>

          <div>
            <label className="text-sm font-medium mb-2 block">Branches</label>
            <div className="flex flex-wrap gap-1">
              {branches.map((b) => (
                <Button
                  key={b.id}
                  variant={selectedBranches.includes(b.id) ? "default" : "outline"}
                  size="sm"
                  onClick={() =>
                    toggleSelection(b.id, selectedBranches, setSelectedBranches)
                  }
                >
                  {b.name}
                </Button>
              ))}
            </div>
          </div>

          <div>
            <label className="text-sm font-medium mb-2 block">Testbeds</label>
            <div className="flex flex-wrap gap-1">
              {testbeds.map((t) => (
                <Button
                  key={t.id}
                  variant={selectedTestbeds.includes(t.id) ? "default" : "outline"}
                  size="sm"
                  onClick={() =>
                    toggleSelection(t.id, selectedTestbeds, setSelectedTestbeds)
                  }
                >
                  {t.name}
                </Button>
              ))}
            </div>
          </div>

          <div>
            <label className="text-sm font-medium mb-2 block">Measures</label>
            <div className="flex flex-wrap gap-1">
              {measures.map((m) => (
                <Button
                  key={m.id}
                  variant={selectedMeasures.includes(m.id) ? "default" : "outline"}
                  size="sm"
                  onClick={() =>
                    toggleSelection(m.id, selectedMeasures, setSelectedMeasures)
                  }
                >
                  {m.name}
                </Button>
              ))}
            </div>
          </div>
        </div>

        {chartData.length === 0 ? (
          <div className="h-[400px] flex items-center justify-center text-muted-foreground">
            No data available for the selected filters
          </div>
        ) : (
          <div className="h-[400px]">
            <ResponsiveContainer width="100%" height="100%">
              <ComposedChart data={chartData}>
                <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
                <XAxis
                  dataKey="date"
                  tick={{ fontSize: 12 }}
                  tickFormatter={(value) =>
                    new Date(value).toLocaleDateString("en-US", {
                      month: "short",
                      day: "numeric",
                    })
                  }
                />
                <YAxis
                  tick={{ fontSize: 12 }}
                  tickFormatter={(value) => formatValue(value)}
                />
                <Tooltip
                  isAnimationActive={false}
                  content={({ active, payload, label }) => {
                    if (!active || !payload?.length) return null;
                    return (
                      <div className="bg-background border rounded-lg shadow-lg p-3 text-sm">
                        <div className="font-medium mb-2">
                          {new Date(label).toLocaleDateString()}
                        </div>
                        {payload.map((item, idx) => (
                          <div
                            key={idx}
                            className="flex items-center gap-2"
                            style={{ color: item.color }}
                          >
                            <span>{item.name}:</span>
                            <span className="font-mono">
                              {formatValue(item.value as number)}
                            </span>
                          </div>
                        ))}
                      </div>
                    );
                  }}
                />
                <Legend />
                {filteredSeries.map((s, idx) => {
                  const seriesKey = getSeriesKey(s);
                  const color = COLORS[idx % COLORS.length];
                  return (
                    <Line
                      key={seriesKey}
                      type="monotone"
                      dataKey={seriesKey}
                      name={getSeriesLabel(s)}
                      stroke={color}
                      strokeWidth={2}
                      dot={{ r: 3 }}
                      activeDot={{ r: 5 }}
                      isAnimationActive={false}
                    />
                  );
                })}
              </ComposedChart>
            </ResponsiveContainer>
          </div>
        )}
      </CardContent>
    </Card>
  );
}

function getSeriesKey(series: Series): string {
  return `${series.benchmark.id}_${series.branch.id}_${series.testbed.id}_${series.measure.id}`;
}

function getSeriesLabel(series: Series): string {
  return `${series.benchmark.name} (${series.branch.name})`;
}

function transformData(series: Series[]): Record<string, unknown>[] {
  const dataMap = new Map<string, Record<string, unknown>>();

  for (const s of series) {
    const key = getSeriesKey(s);
    for (const point of s.data) {
      const dateKey = point.x;
      if (!dataMap.has(dateKey)) {
        dataMap.set(dateKey, { date: dateKey });
      }
      const entry = dataMap.get(dateKey)!;
      entry[key] = point.y;
      entry[`${key}_lower`] = point.lower;
      entry[`${key}_upper`] = point.upper;
      entry[`${key}_hash`] = point.gitHash;
    }
  }

  return Array.from(dataMap.values()).sort(
    (a, b) => new Date(a.date as string).getTime() - new Date(b.date as string).getTime()
  );
}

function formatValue(value: number): string {
  if (value >= 1e9) {
    return `${(value / 1e9).toFixed(2)}B`;
  } else if (value >= 1e6) {
    return `${(value / 1e6).toFixed(2)}M`;
  } else if (value >= 1e3) {
    return `${(value / 1e3).toFixed(2)}K`;
  } else if (value < 1) {
    return value.toFixed(4);
  }
  return value.toFixed(2);
}
