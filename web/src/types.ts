export interface Todo {
  id: string;
  text: string;
  completed: boolean;
  createdAt: number;
  completedAt?: number;
  deadline?: number;
  order: number;
  groupId: string;
  priority: number;
  color?: string;
  updatedAt: number;
  isDeleted?: boolean;
}

export interface TodoGroup {
  id: string;
  name: string;
  order: number;
  collapsed: boolean;
  updatedAt: number;
}

export interface DateInfo {
  solar_date: string;
  lunar_date: string;
  weekday: string;
  lunar_year: string;
  lunar_month: string;
  lunar_day: string;
}
