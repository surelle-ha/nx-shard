export default defineNuxtRouteMiddleware(async (to, from) => {
  const accountStore = useAccountStore();

  if (to.path === "/auth/login" || to.path === "/auth/register") {
    return;
  }

  if (!accountStore.account) {
    try {
      await accountStore.initializeUser();
    } catch (error) {
      console.error("Failed to initialize user:", error);
      return navigateTo("/auth/login");
    }
  }

  if (!accountStore.isAuthenticated) {
    return navigateTo("/auth/login");
  }
});
