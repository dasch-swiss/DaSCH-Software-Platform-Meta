import { navigate } from 'svelte-routing';
import { readable, writable } from 'svelte/store';
import type { Metadata, PaginationData, ProjectMetadata } from './interfaces';

export const pagination = writable({} as PaginationData);
export const pagedResults = writable(undefined as ProjectMetadata[]);
export const projectMetadata = writable(undefined as Metadata);
export const query = writable('');
export const previousRoute = writable('');
export const handleSnackbar = writable({ isSnackbar: false, message: '' });
export const statusFilter = writable({ showOngoing: true, showFinished: true });

export const isTestEnvironment = readable(
window.location.hostname === 'localhost' || window.location.hostname.startsWith('meta.test') || window.location.hostname.startsWith('meta.dev')
);

export function baseUrl() {
  const protocol = window.location.protocol;
  const port = protocol === 'https:' ? '' : ':3000';
  return `${protocol}//${window.location.hostname}${port}`;
}

export async function getProjectsMetadata(page: number, q?: string): Promise<void> {
  // const baseUrl = process.env.BASE_URL;
  const baseResultsRange = [1, 9];
  let route: string;
  let currentResultsRange = baseResultsRange.map(v => v + ((page - 1) * baseResultsRange[1]));

  if (q) {
    query.set(q);
    route = `projects?q=${q}&_page=${page}&_limit=${baseResultsRange[1]}`;
    handleSnackbar.set({ isSnackbar: true, message: `Displaying search results for query: ${q}` });
  } else {
    query.set('');
    route = `projects?_page=${page}&_limit=${baseResultsRange[1]}`;
  }

  statusFilter.subscribe(f => {
    if (!f.showFinished || !f.showOngoing) {
      const filter = `&filter=${!f.showOngoing ? 'o' : ''}${!f.showFinished ? 'f' : ''}`;
      route += filter;
    }
  });

  navigate(`/${route}`);

  await fetch(`${(baseUrl())}/api/v1/${route}`)
    .then(r => {
      console.log("RESPONSE count header:", r.headers.get('X-Total-Count'));
      const totalCount = parseInt(r.headers.get('X-Total-Count'));
      let totalPages = Math.floor(totalCount / baseResultsRange[1]);
      if (!Number.isInteger(totalCount / baseResultsRange[1])) {
        totalPages++;
      }
      ;
      pagination.set({ currentPage: page, currentResultsRange, totalCount, totalPages });
      return r.json();
    })
    .then(data => {
      pagedResults.set(data), console.log(data);
    });
}
