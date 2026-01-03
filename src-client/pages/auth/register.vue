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
      name: "confirmPassword",
      label: "Confirm Password",
      type: "password",
      placeholder: "Enter your password again",
      required: true,
    },
  ];
  
  const schema = z.object({
    email: z.string().email("Invalid email"),
    password: z.string().min(8, "Must be at least 8 characters"),
    confirmPassword: z.string().min(8, "Must be at least 8 characters"),
  }).refine((data) => data.password === data.confirmPassword, {
    message: "Passwords don't match",
    path: ["confirmPassword"],
  });
  
  type Schema = z.output<typeof schema>;
  
  async function onSubmit(payload: FormSubmitEvent<Schema>) {
    loading.value = true;
  
    try {
      // Register the user with Supabase
      const supabase = useSupabase();
      const { data, error } = await supabase.auth.signUp({
        email: payload.data.email,
        password: payload.data.password,
      });
  
      if (error) throw error;
  
      toast.add({
        title: "Success",
        description: "Registration successful! Please wait for your account to be approved.",
        color: "primary",
      });
  
      // Optionally redirect to login or wait for email verification
      navigateTo("/auth/login");
    } catch (error: any) {
      toast.add({
        title: "Error",
        description: error.message || "Failed to register",
        color: "error",
      });
    } finally {
      loading.value = false;
    }
  }
  </script>
  
  <template>
    <div class="space-y-4">
      <UPageCard class="w-full max-w-md">
        <UAuthForm
          :schema="schema"
          title="Register"
          description="Create a new account to get started."
          icon="i-lucide-user-plus"
          :fields="fields"
          :loading="loading"
          submit-button="{ label: 'Create Account' }"
          @submit="onSubmit"
        >
          <template #description>
            Already have an account?
            <ULink to="/auth/login" class="text-primary font-medium">Sign In</ULink>.
          </template>
          <template #footer>
            By signing up, you agree to our
            <ULink to="#" class="text-primary font-medium">Terms of Service</ULink>.
          </template>
        </UAuthForm>
      </UPageCard>
  
      <UPageCard class="w-full max-w-md">
        <UAlert
          title="⚠️ Notice!"
          description="Access to nxShard requires developer approval. Please allow up to 24 hours for the approval process."
          variant="subtle"
          color="warning"
        />
      </UPageCard>
    </div>
  </template>