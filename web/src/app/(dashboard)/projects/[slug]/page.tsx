import { notFound, redirect } from "next/navigation";
import Link from "next/link";
import { Suspense } from "react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Skeleton } from "@/components/ui/skeleton";
import { getSession, createGraphQLToken } from "@/lib/auth";
import { createAuthenticatedClient } from "@/lib/graphql/client";
import { PROJECT_QUERY, ALERTS_QUERY } from "@/lib/graphql/queries";
import { BenchmarkChart } from "@/components/benchmark-chart-server";

interface Branch {
  id: string;
  name: string;
}

interface Testbed {
  id: string;
  name: string;
}

interface Measure {
  id: string;
  name: string;
  units: string | null;
}

interface Benchmark {
  id: string;
  name: string;
}

interface Report {
  id: string;
  gitHash: string | null;
  createdAt: string;
  branch: Branch;
  testbed: Testbed;
}

interface Threshold {
  id: string;
  branchId: string | null;
  testbedId: string | null;
  measureId: string;
  upperBoundary: number | null;
  lowerBoundary: number | null;
  minSampleSize: number;
}

interface Alert {
  id: string;
  baselineValue: number;
  percentChange: number;
  status: string;
  createdAt: string;
  metric: {
    id: string;
    value: number;
    benchmark: { id: string; name: string };
    measure: { id: string; name: string };
  };
}

interface Project {
  id: string;
  slug: string;
  name: string;
  description: string | null;
  public: boolean;
  createdAt: string;
  branches: Branch[];
  testbeds: Testbed[];
  measures: Measure[];
  benchmarks: Benchmark[];
  thresholds: Threshold[];
  recentReports: Report[];
}

async function getProject(slug: string): Promise<Project | null> {
  const session = await getSession();
  if (!session) {
    return null;
  }

  const token = await createGraphQLToken(session);
  const client = createAuthenticatedClient(token);
  const result = await client.query(PROJECT_QUERY, { slug });

  if (result.error) {
    console.error("GraphQL error:", result.error);
    return null;
  }

  return result.data?.project || null;
}

async function getAlerts(projectSlug: string): Promise<Alert[]> {
  const session = await getSession();
  if (!session) {
    return [];
  }

  const token = await createGraphQLToken(session);
  const client = createAuthenticatedClient(token);
  const result = await client.query(ALERTS_QUERY, { projectSlug, status: "ACTIVE" });

  if (result.error) {
    console.error("GraphQL error:", result.error);
    return [];
  }

  return result.data?.alerts || [];
}

export default async function ProjectPage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const session = await getSession();
  if (!session) {
    redirect("/login");
  }

  const { slug } = await params;
  const [project, alerts] = await Promise.all([
    getProject(slug),
    getAlerts(slug),
  ]);

  if (!project) {
    notFound();
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <div className="flex items-center gap-2 text-sm text-muted-foreground mb-1">
            <Link href="/projects" className="hover:underline">
              Projects
            </Link>
            <span>/</span>
            <span>{project.slug}</span>
          </div>
          <h1 className="text-3xl font-bold">{project.name}</h1>
          {project.description && (
            <p className="text-muted-foreground mt-1">{project.description}</p>
          )}
        </div>
        <div className="flex gap-2">
          <Button variant="outline" asChild>
            <Link href={`/projects/${project.slug}/settings`}>Settings</Link>
          </Button>
        </div>
      </div>

      {alerts.length > 0 && (
        <Card className="border-destructive">
          <CardHeader className="pb-3">
            <CardTitle className="text-destructive flex items-center gap-2">
              <span className="h-2 w-2 rounded-full bg-destructive animate-pulse" />
              {alerts.length} Active Alert{alerts.length > 1 ? "s" : ""}
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {alerts.slice(0, 3).map((alert) => (
                <div
                  key={alert.id}
                  className="flex items-center justify-between text-sm"
                >
                  <span>
                    {alert.metric.benchmark.name} / {alert.metric.measure.name}
                  </span>
                  <span className="text-destructive font-mono">
                    {alert.percentChange > 0 ? "+" : ""}
                    {alert.percentChange.toFixed(1)}%
                  </span>
                </div>
              ))}
              {alerts.length > 3 && (
                <p className="text-xs text-muted-foreground">
                  and {alerts.length - 3} more...
                </p>
              )}
            </div>
          </CardContent>
        </Card>
      )}

      <Tabs defaultValue="perf" className="space-y-4">
        <TabsList>
          <TabsTrigger value="perf">Performance</TabsTrigger>
          <TabsTrigger value="reports">Reports</TabsTrigger>
          <TabsTrigger value="thresholds">Thresholds</TabsTrigger>
        </TabsList>

        <TabsContent value="perf" className="space-y-4">
          {project.benchmarks.length === 0 ? (
            <Card>
              <CardHeader>
                <CardTitle>No benchmarks yet</CardTitle>
                <CardDescription>
                  Submit your first benchmark report to see performance data
                </CardDescription>
              </CardHeader>
              <CardContent>
                <pre className="bg-muted p-4 rounded-lg text-sm overflow-x-auto">
                  <code>
{`# Install the CLI
cargo install rabbitbench

# Configure with your API token
rabbitbench auth login

# Run benchmarks and submit
cargo bench -- --save-baseline main | rabbitbench run \\
  --project ${project.slug} \\
  --branch main \\
  --testbed local`}
                  </code>
                </pre>
              </CardContent>
            </Card>
          ) : (
            <Suspense fallback={<Skeleton className="h-[500px] w-full" />}>
              <BenchmarkChart
                projectSlug={project.slug}
                benchmarks={project.benchmarks}
                branches={project.branches}
                testbeds={project.testbeds}
                measures={project.measures}
              />
            </Suspense>
          )}
        </TabsContent>

        <TabsContent value="reports" className="space-y-4">
          {project.recentReports.length === 0 ? (
            <Card>
              <CardHeader>
                <CardTitle>No reports yet</CardTitle>
                <CardDescription>
                  Reports will appear here once you submit benchmark data
                </CardDescription>
              </CardHeader>
            </Card>
          ) : (
            <Card>
              <CardHeader>
                <CardTitle>Recent Reports</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-2">
                  {project.recentReports.map((report) => (
                    <div
                      key={report.id}
                      className="flex items-center justify-between py-2 border-b last:border-0"
                    >
                      <div>
                        <div className="font-mono text-sm">
                          {report.gitHash?.slice(0, 7) || "N/A"}
                        </div>
                        <div className="text-xs text-muted-foreground">
                          {report.branch.name} / {report.testbed.name}
                        </div>
                      </div>
                      <div className="text-xs text-muted-foreground">
                        {new Date(report.createdAt).toLocaleDateString()}
                      </div>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          )}
        </TabsContent>

        <TabsContent value="thresholds" className="space-y-4">
          <div className="flex justify-between items-center">
            <div>
              <h3 className="text-lg font-semibold">Thresholds</h3>
              <p className="text-sm text-muted-foreground">
                Configure regression detection rules
              </p>
            </div>
            <Button variant="outline" asChild>
              <Link href={`/projects/${project.slug}/thresholds/new`}>
                Add Threshold
              </Link>
            </Button>
          </div>

          {project.thresholds.length === 0 ? (
            <Card>
              <CardHeader>
                <CardTitle>No thresholds configured</CardTitle>
                <CardDescription>
                  Add thresholds to automatically detect performance regressions
                </CardDescription>
              </CardHeader>
            </Card>
          ) : (
            <div className="grid gap-4">
              {project.thresholds.map((threshold) => {
                const measure = project.measures.find(
                  (m) => m.id === threshold.measureId
                );
                const branch = threshold.branchId
                  ? project.branches.find((b) => b.id === threshold.branchId)
                  : null;
                const testbed = threshold.testbedId
                  ? project.testbeds.find((t) => t.id === threshold.testbedId)
                  : null;

                return (
                  <Card key={threshold.id}>
                    <CardHeader className="pb-2">
                      <CardTitle className="text-base">
                        {measure?.name || "Unknown measure"}
                      </CardTitle>
                      <CardDescription>
                        {branch?.name || "All branches"} /{" "}
                        {testbed?.name || "All testbeds"}
                      </CardDescription>
                    </CardHeader>
                    <CardContent>
                      <div className="flex gap-4 text-sm">
                        {threshold.upperBoundary && (
                          <div>
                            <span className="text-muted-foreground">Upper: </span>
                            <span className="font-mono">
                              +{threshold.upperBoundary}%
                            </span>
                          </div>
                        )}
                        {threshold.lowerBoundary && (
                          <div>
                            <span className="text-muted-foreground">Lower: </span>
                            <span className="font-mono">
                              -{threshold.lowerBoundary}%
                            </span>
                          </div>
                        )}
                        <div>
                          <span className="text-muted-foreground">
                            Min samples:{" "}
                          </span>
                          <span className="font-mono">
                            {threshold.minSampleSize}
                          </span>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                );
              })}
            </div>
          )}
        </TabsContent>
      </Tabs>
    </div>
  );
}
