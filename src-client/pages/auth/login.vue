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
  password: z
    .string()
    .min(8, "Must be at least 8 characters"),
});

type Schema = z.output<typeof schema>;

async function onSubmit(payload: FormSubmitEvent<Schema>) {
  loading.value = true;

  try {
    await accountStore.login(payload.data.email, payload.data.password);

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
    />
  </UPageCard>
</template>
