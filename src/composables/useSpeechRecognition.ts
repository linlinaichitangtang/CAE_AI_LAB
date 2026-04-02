import { ref, onUnmounted } from 'vue'

export interface SpeechRecognitionResult {
  transcript: string
  confidence: number
  isFinal: boolean
}

export function useSpeechRecognition() {
  const isListening = ref(false)
  const isSupported = ref(false)
  const transcript = ref('')
  const confidence = ref(0)
  const error = ref<string | null>(null)

  let recognition: any = null

  // 检测浏览器支持
  const SpeechRecognition = (typeof window !== 'undefined')
    ? ((window as any).SpeechRecognition || (window as any).webkitSpeechRecognition)
    : null
  isSupported.value = !!SpeechRecognition

  function start(lang: string = 'zh-CN') {
    if (!SpeechRecognition) {
      error.value = '浏览器不支持语音识别'
      return
    }

    // 先停止之前的实例
    stop()

    recognition = new SpeechRecognition()
    recognition.lang = lang
    recognition.continuous = true
    recognition.interimResults = true

    recognition.onresult = (event: any) => {
      let finalTranscript = ''
      let interimTranscript = ''

      for (let i = event.resultIndex; i < event.results.length; i++) {
        const result = event.results[i]
        if (result.isFinal) {
          finalTranscript += result[0].transcript
          confidence.value = result[0].confidence
        } else {
          interimTranscript += result[0].transcript
        }
      }

      transcript.value = finalTranscript || interimTranscript
    }

    recognition.onerror = (event: any) => {
      error.value = event.error
      isListening.value = false
    }

    recognition.onend = () => {
      isListening.value = false
    }

    try {
      recognition.start()
      isListening.value = true
      error.value = null
    } catch (e: any) {
      error.value = `启动语音识别失败: ${e.message || e}`
      isListening.value = false
    }
  }

  function stop() {
    if (recognition) {
      try {
        recognition.stop()
      } catch {
        // 忽略停止时的错误
      }
      recognition = null
    }
    isListening.value = false
  }

  function toggle(lang?: string) {
    if (isListening.value) {
      stop()
    } else {
      start(lang)
    }
  }

  function clearTranscript() {
    transcript.value = ''
    confidence.value = 0
  }

  onUnmounted(() => {
    stop()
  })

  return {
    isListening,
    isSupported,
    transcript,
    confidence,
    error,
    start,
    stop,
    toggle,
    clearTranscript,
  }
}
