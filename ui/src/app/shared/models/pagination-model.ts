export interface Pagination<T> {
  currentPage: number;
  pageSize: number;
  totla: number;
  pages: number;
  items: T[];
}
