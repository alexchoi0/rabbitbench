"use client";

import { signIn } from "@/lib/auth-client";
import { Button } from "@/components/ui/button";

export function LoginButton() {
  const handleLogin = () => {
    signIn.social({
      provider: "google",
      callbackURL: "/projects",
    });
  };

  return (
    <Button onClick={handleLogin} size="lg">
      Sign in with Google
    </Button>
  );
}
