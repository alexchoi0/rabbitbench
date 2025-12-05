import Link from "next/link";
import { redirect } from "next/navigation";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { getSession, createGraphQLToken } from "@/lib/auth";
import { createAuthenticatedClient } from "@/lib/graphql/client";
import { PROJECTS_QUERY } from "@/lib/graphql/queries";

interface Project {
  id: string;
  slug: string;
  name: string;
  description: string | null;
  public: boolean;
  createdAt: string;
}

async function getProjects(): Promise<Project[]> {
  const session = await getSession();
  if (!session) {
    return [];
  }

  const token = await createGraphQLToken(session);
  const client = createAuthenticatedClient(token);
  const result = await client.query(PROJECTS_QUERY, {});

  if (result.error) {
    console.error("GraphQL error:", result.error);
    return [];
  }

  return result.data?.projects || [];
}

export default async function ProjectsPage() {
  const session = await getSession();

  if (!session) {
    redirect("/login");
  }

  const projects = await getProjects();

  return (
    <div>
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold">Projects</h1>
          <p className="text-muted-foreground">
            Manage your benchmark projects
          </p>
        </div>
        <Button asChild>
          <Link href="/projects/new">New Project</Link>
        </Button>
      </div>

      {projects.length === 0 ? (
        <Card className="mt-8">
          <CardHeader>
            <CardTitle>No projects yet</CardTitle>
            <CardDescription>
              Create your first project to start tracking benchmarks
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Button asChild>
              <Link href="/projects/new">Create Project</Link>
            </Button>
          </CardContent>
        </Card>
      ) : (
        <div className="mt-8 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {projects.map((project) => (
            <Link key={project.id} href={`/projects/${project.slug}`}>
              <Card className="h-full transition-colors hover:bg-muted/50">
                <CardHeader>
                  <CardTitle className="flex items-center justify-between">
                    {project.name}
                    {project.public && (
                      <span className="text-xs font-normal text-muted-foreground">
                        Public
                      </span>
                    )}
                  </CardTitle>
                  <CardDescription>{project.slug}</CardDescription>
                </CardHeader>
                {project.description && (
                  <CardContent>
                    <p className="text-sm text-muted-foreground line-clamp-2">
                      {project.description}
                    </p>
                  </CardContent>
                )}
              </Card>
            </Link>
          ))}
        </div>
      )}
    </div>
  );
}
