/**
 * WebSocket 客户端 —— 监听服务端数据更新通知，自动刷新页面数据。
 */

type MessageHandler = (data: any) => void;

interface WSState {
  ws: WebSocket | null;
  reconnectTimer: number | null;
  messageHandlers: Set<MessageHandler>;
}

const state: WSState = {
  ws: null,
  reconnectTimer: null,
  messageHandlers: new Set(),
};

function getWsUrl(): string {
  const protocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
  return `${protocol}//${location.host}/ws`;
}

function connect() {
  // Clean up any existing connection
  disconnect();

  try {
    const url = getWsUrl();
    console.log('[WS] 连接:', url);
    const ws = new WebSocket(url);

    ws.onopen = () => {
      console.log('[WS] 已连接');
      // Clear any pending reconnect since we connected successfully
      if (state.reconnectTimer !== null) {
        clearTimeout(state.reconnectTimer);
        state.reconnectTimer = null;
      }
    };

    ws.onmessage = (event) => {
      try {
        const msg = JSON.parse(event.data);
        if (msg.type === 'data_updated') {
          console.log('[WS] 收到数据更新通知');
          state.messageHandlers.forEach((fn) => fn(msg));
        }
      } catch {
        // Not JSON, ignore
      }
    };

    ws.onclose = () => {
      console.log('[WS] 连接关闭，5秒后重连');
      scheduleReconnect();
    };

    ws.onerror = () => {
      console.error('[WS] 连接错误');
      ws.close();
    };

    state.ws = ws;
  } catch (err) {
    console.error('[WS] 连接失败:', err);
    scheduleReconnect();
  }
}

function scheduleReconnect() {
  if (state.reconnectTimer !== null) return;
  state.reconnectTimer = window.setTimeout(() => {
    state.reconnectTimer = null;
    connect();
  }, 5000);
}

function disconnect() {
  if (state.reconnectTimer !== null) {
    clearTimeout(state.reconnectTimer);
    state.reconnectTimer = null;
  }
  if (state.ws) {
    state.ws.onclose = null; // prevent reconnect trigger
    state.ws.close();
    state.ws = null;
  }
}

/**
 * Register a handler that fires when "data_updated" message is received.
 * Returns an unsubscribe function.
 */
export function onDataUpdated(handler: MessageHandler): () => void {
  state.messageHandlers.add(handler);
  return () => {
    state.messageHandlers.delete(handler);
  };
}

/**
 * Start the WebSocket connection.
 */
export function startWS() {
  connect();
}

/**
 * Stop the WebSocket connection and clean up.
 */
export function stopWS() {
  disconnect();
  state.messageHandlers.clear();
}
