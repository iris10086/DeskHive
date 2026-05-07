export interface Todo {
  id: string; // 添加唯一ID
  text: string;
  completed: boolean;
  createdAt: number; // Unix时间戳（秒）
  completedAt?: number; // 完成时间，Unix时间戳（秒），可选
  deadline?: number; // 截止时间，Unix时间戳（秒），可选
  order: number; // 在分组内的排序
  groupId: string; // 所属分组ID
  priority: number; // 优先级：0=普通，1=重要
  color?: string; // 小圆点颜色，可选（已弃用，由优先级决定）
  updatedAt: number; // 最后更新时间，Unix时间戳（秒），用于同步冲突检测
  isDeleted?: boolean; // 逻辑删除标记，用于同步
}

export interface TodoGroup {
  id: string;
  name: string;
  order: number; // 分组排序
  collapsed: boolean; // 是否折叠
  updatedAt: number; // 最后更新时间，Unix时间戳（秒），用于同步冲突检测
}

export interface DateInfo {
  solar_date: string;    // 公历日期
  lunar_date: string;    // 农历日期
  weekday: string;       // 星期
  lunar_year: string;    // 农历年份
  lunar_month: string;   // 农历月份
  lunar_day: string;     // 农历日期
}

export interface AppSettings {
  opacity: number;
  disable_drag: boolean;
  auto_start: boolean;
  silent_start: boolean;
  theme: string;
  priority_color: string;
  window_level: string;
  timeline_deadline_priority: boolean;
  enable_deadline_notification: boolean;
  notification_minutes_before: number;
  window_size: string; // 'small' | 'medium' | 'large'
}