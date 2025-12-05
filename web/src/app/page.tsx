import Link from "next/link";
import { Button } from "@/components/ui/button";
import { UserMenu } from "@/components/auth/user-menu";

export default function Home() {
  return (
    <div className="min-h-screen bg-background">
      <header className="border-b">
        <div className="container mx-auto flex h-16 items-center justify-between px-4">
          <Link href="/" className="text-xl font-bold">
            RabbitBench
          </Link>
          <UserMenu />
        </div>
      </header>

      <main className="container mx-auto px-4 py-24">
        <div className="flex flex-col items-center text-center">
          <h1 className="text-4xl font-bold tracking-tight sm:text-6xl">
            Track Your Benchmark Performance
          </h1>
          <p className="mt-6 max-w-2xl text-lg text-muted-foreground">
            Continuous benchmarking for Rust projects. Collect, visualize, and
            catch performance regressions in your Criterion benchmarks.
          </p>
          <div className="mt-10 flex gap-4">
            <Button asChild size="lg">
              <Link href="/login">Get Started</Link>
            </Button>
            <Button asChild variant="outline" size="lg">
              <a
                href="https://github.com/yourusername/rabbitbench"
                target="_blank"
                rel="noopener noreferrer"
              >
                View on GitHub
              </a>
            </Button>
          </div>
        </div>

        <div className="mt-32 grid gap-8 sm:grid-cols-3">
          <div className="rounded-lg border p-6">
            <h3 className="text-lg font-semibold">Collect</h3>
            <p className="mt-2 text-muted-foreground">
              Run your Criterion benchmarks with our CLI and automatically
              submit results to track over time.
            </p>
          </div>
          <div className="rounded-lg border p-6">
            <h3 className="text-lg font-semibold">Visualize</h3>
            <p className="mt-2 text-muted-foreground">
              Interactive charts show performance trends across branches,
              commits, and testbeds.
            </p>
          </div>
          <div className="rounded-lg border p-6">
            <h3 className="text-lg font-semibold">Alert</h3>
            <p className="mt-2 text-muted-foreground">
              Configure thresholds and get notified when benchmarks regress
              beyond acceptable limits.
            </p>
          </div>
        </div>

        <div className="mt-32">
          <h2 className="text-center text-2xl font-bold">Easy to Use CLI</h2>
          <div className="mt-8 mx-auto max-w-2xl rounded-lg bg-zinc-950 p-6 text-zinc-50">
            <pre className="text-sm overflow-x-auto">
              <code>{`# Install the CLI
cargo install rabbitbench

# Authenticate
rabbitbench auth login --token <your-token>

# Run benchmarks and submit
rabbitbench run --project my-lib --branch main "cargo bench"`}</code>
            </pre>
          </div>
        </div>
      </main>

      <footer className="border-t py-8">
        <div className="container mx-auto px-4 text-center text-sm text-muted-foreground">
          Built for the Rust community
        </div>
      </footer>
    </div>
  );
}
