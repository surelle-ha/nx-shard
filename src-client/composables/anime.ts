export const useAnimeApi = () => {
  const baseUrl = "http://localhost:3030/api/v1";

  const request = async (path: string) => {
    const res = await fetch(`${baseUrl}${path}`);
    if (!res.ok) throw new Error("Failed to fetch data");
    return await res.json();
  };

  return {
    request,
    home: () => request("/home"),
    spotlight: () => request("/spotlight"),
    topten: () => request("/topten"),
    animeList: (query: string, page = 1) => request(`/${query}?page=${page}`),
    azList: (letter: string, page = 1) =>
      request(`/az-list/${letter}?page=${page}`),
    genreList: (genre: string, page = 1) =>
      request(`/genre/${genre}?page=${page}`),
    animeDetails: (animeId: string) => request(`/anime/${animeId}`),
    search: (keyword: string, page = 1) =>
      request(`/search?keyword=${keyword}&page=${page}`),
    searchSuggestion: (keyword: string) =>
      request(`/search/suggestion?keyword=${keyword}`),
    episodes: (animeId: string) => request(`/episodes/${animeId}`),
    servers: (episodeId: string) => request(`/servers?id=${episodeId}`),
    stream: (episodeId: string, server: string, type: string) =>
      request(`/stream?id=${episodeId}&server=${server}&type=${type}`),
  };
};
