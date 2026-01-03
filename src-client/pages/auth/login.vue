<script setup lang="ts">
definePageMeta({
  layout: "authentication",
});

import * as z from "zod";
import type { FormSubmitEvent, AuthFormField } from "@nuxt/ui";
import { useAccountStore } from "~/stores/account";

const accountStore = useAccountStore();
const toast = useToast();
const loading = ref(false);

const fields: AuthFormField[] = [
  {
    name: "email",
    type: "email",
    label: "Email",
    placeholder: "Enter your email",
    required: true,
  },
  {
    name: "password",
    label: "Password",
    type: "password",
    placeholder: "Enter your password",
    required: true,
  },
  {
    name: "remember",
    label: "Remember me",
    type: "checkbox",
    description: "You will be logged in for 30 days.",
  },
];

const schema = z.object({
  email: z.string().email("Invalid email"),
  password: z.string().min(8, "Must be at least 8 characters"),
});

type Schema = z.output<typeof schema>;

async function onSubmit(payload: FormSubmitEvent<Schema>) {
  loading.value = true;

  try {
    await accountStore.login(payload.data.email, payload.data.password);
    // if (accountStore.account?.isNew) return navigateTo("/auth/onboard");
    navigateTo("/");
  } catch (error: any) {
    toast.add({
      title: "Error",
      description: error.message || "Failed to login",
      color: "error",
    });
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <UPageCard class="w-full max-w-md">
    <UAuthForm
      :schema="schema"
      title="Login"
      description="Enter your credentials to access your account."
      icon="i-lucide-user"
      :fields="fields"
      :loading="loading"
      @submit="onSubmit"
    >
      <template #description>
        Don't have an account?
        <ULink to="/auth/register" class="text-primary font-medium">Sign up</ULink>.
      </template>
      <template #password-hint>
        <ULink to="#" class="hidden text-primary font-medium" tabindex="-1"
          >Forgot password?</ULink
        >
      </template>
      <template #footer>
        By signing in, you agree to our
        <ULink to="#" class="text-primary font-medium">Terms of Service</ULink>.
      </template>
    </UAuthForm>
  </UPageCard>
</template>
