"use server";

import { createAuthenticatedClient } from "@/lib/graphql/client";
import { CREATE_PROJECT_MUTATION } from "@/lib/graphql/mutations";
import { getSession, createGraphQLToken } from "@/lib/auth";
import { revalidatePath } from "next/cache";

export async function createProject(formData: {
  slug: string;
  name: string;
  description?: string;
}): Promise<{ success: boolean; slug?: string; error?: string }> {
  const session = await getSession();

  if (!session?.user) {
    return { success: false, error: "Not authenticated" };
  }

  const token = await createGraphQLToken(session);
  const client = createAuthenticatedClient(token);

  const result = await client.mutation(CREATE_PROJECT_MUTATION, {
    input: {
      slug: formData.slug,
      name: formData.name,
      description: formData.description || null,
    },
  });

  if (result.error) {
    console.error("GraphQL error:", result.error);
    return { success: false, error: result.error.message };
  }

  revalidatePath("/projects");
  return { success: true, slug: result.data.createProject.slug };
}
