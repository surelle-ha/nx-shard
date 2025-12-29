export default defineNuxtRouteMiddleware((to, from) => {
  const accountStore = useAccountStore();

  if (to.path === "/auth/login" || to.path === "/auth/register") {
    return;
  }

  if (!accountStore.isAuthenticated) {
    return navigateTo("/auth/login");
  }
});
