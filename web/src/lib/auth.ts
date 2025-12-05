import { betterAuth } from "better-auth";
import { headers } from "next/headers";
import * as crypto from "crypto";

const DEV_MODE = process.env.DEV_MODE === "true" || process.env.DEV_MODE === "1";

export const auth = betterAuth({
  socialProviders: {
    google: {
      clientId: process.env.GOOGLE_CLIENT_ID || "",
      clientSecret: process.env.GOOGLE_CLIENT_SECRET || "",
    },
  },
  session: {
    expiresIn: 60 * 60 * 24 * 7, // 7 days
    updateAge: 60 * 60 * 24, // 1 day
  },
  secret: process.env.AUTH_SECRET!,
});

const devSession = {
  user: {
    id: "dev-user-id",
    email: "dev@localhost",
    name: "Dev User",
    image: null,
  },
  session: {
    id: "dev-session",
    userId: "dev-user-id",
    expiresAt: new Date(Date.now() + 1000 * 60 * 60 * 24 * 7),
  },
};

export async function getSession() {
  if (DEV_MODE) {
    return devSession;
  }
  const headersList = await headers();
  return auth.api.getSession({
    headers: headersList,
  });
}

export async function createGraphQLToken(session: {
  user: { id: string; email: string; name?: string | null; image?: string | null };
}) {
  const header = { alg: "HS256", typ: "JWT" };
  const now = Math.floor(Date.now() / 1000);
  const payload = {
    sub: session.user.id,
    email: session.user.email,
    name: session.user.name || undefined,
    picture: session.user.image || undefined,
    iat: now,
    exp: now + 3600, // 1 hour
  };

  const base64UrlEncode = (data: string) =>
    Buffer.from(data).toString("base64url");

  const headerEncoded = base64UrlEncode(JSON.stringify(header));
  const payloadEncoded = base64UrlEncode(JSON.stringify(payload));
  const signatureInput = `${headerEncoded}.${payloadEncoded}`;

  const signature = crypto
    .createHmac("sha256", process.env.AUTH_SECRET!)
    .update(signatureInput)
    .digest("base64url");

  return `${signatureInput}.${signature}`;
}
