import { ref } from 'vue'

enum MessageAction {
  Task = "Task",
  Result = "Result",
}

interface CalcTaskMessage {
  action: MessageAction,
  content: string,
}

export class CalcTask {

  _socket: WebSocket | null = null

  task = ref<string | undefined>()
  correctResult = ref(false)
  resultAvailableCallback: ((isCorrect: boolean) => void) | undefined;

  constructor(websocketUri: string) {
  
    this._socket = new WebSocket(websocketUri)
  
    this._socket.onopen = () => {
      this.requestNewTask()
    }
  
    this._socket.onmessage = (ev) => {
      let msg: CalcTaskMessage | null = JSON.parse(ev.data)

      if (msg) {

        if (msg.action === MessageAction.Task) {
          this.task.value = msg.content
        } 
        else if (msg.action === MessageAction.Result) {
          this.resultAvailableCallback?.(msg.content === "correct")
        }

      } else {
        console.error("error parsing web socket message")
      }
    }
  
    this._socket.onclose = () => {
      this._socket = null
    }
  }

  resultAvailable(callback: (isCorrect: boolean) => void) {
    this.resultAvailableCallback = callback
  }

  requestNewTask(): void {
    this._socket?.send(JSON.stringify({
      action: MessageAction.Task,
      content: "",
    } as CalcTaskMessage))
  }

  submitResult(result: number): void {
    this._socket?.send(JSON.stringify({
      action: MessageAction.Result,
      content: result.toString(),
    } as CalcTaskMessage))
  }
}


