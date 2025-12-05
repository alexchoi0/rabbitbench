import { Client, cacheExchange, fetchExchange } from "@urql/core";
import { registerUrql } from "@urql/next/rsc";

const GRAPHQL_URL = process.env.GRAPHQL_URL || "http://localhost:8080/graphql";

const makeClient = () => {
  return new Client({
    url: GRAPHQL_URL,
    exchanges: [cacheExchange, fetchExchange],
    fetchOptions: {
      method: "POST",
    },
  });
};

export const { getClient } = registerUrql(makeClient);

export const createAuthenticatedClient = (token: string) => {
  return new Client({
    url: GRAPHQL_URL,
    exchanges: [fetchExchange],
    preferGetMethod: false,
    fetchOptions: {
      method: "POST",
      headers: {
        authorization: `Bearer ${token}`,
        "Content-Type": "application/json",
      },
      cache: "no-store" as RequestCache,
    },
  });
};

