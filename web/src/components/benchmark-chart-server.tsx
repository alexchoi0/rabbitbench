import { getSession, createGraphQLToken } from "@/lib/auth";
import { createAuthenticatedClient } from "@/lib/graphql/client";
import { PERF_QUERY } from "@/lib/graphql/queries";
import {
  BenchmarkChartClient,
  type Branch,
  type Testbed,
  type Measure,
  type Benchmark,
  type Series,
} from "./benchmark-chart";

interface BenchmarkChartProps {
  projectSlug: string;
  benchmarks: Benchmark[];
  branches: Branch[];
  testbeds: Testbed[];
  measures: Measure[];
}

export async function BenchmarkChart({
  projectSlug,
  benchmarks,
  branches,
  testbeds,
  measures,
}: BenchmarkChartProps) {
  const session = await getSession();

  let series: Series[] = [];

  if (session && benchmarks.length && branches.length && testbeds.length && measures.length) {
    const token = await createGraphQLToken(session);
    const client = createAuthenticatedClient(token);

    const result = await client.query(PERF_QUERY, {
      projectSlug,
      benchmarks: benchmarks.map((b) => b.id),
      branches: branches.map((b) => b.id),
      testbeds: testbeds.map((t) => t.id),
      measures: measures.map((m) => m.id),
    });

    if (!result.error && result.data?.perf?.series) {
      series = result.data.perf.series;
    }
  }

  return (
    <BenchmarkChartClient
      benchmarks={benchmarks}
      branches={branches}
      testbeds={testbeds}
      measures={measures}
      initialSeries={series}
    />
  );
}
