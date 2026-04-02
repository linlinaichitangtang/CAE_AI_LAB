import { createI18n } from 'vue-i18n'
import zhCN from './zh-CN'
import enUS from './en-US'
import jaJP from './ja-JP'
import deDE from './de-DE'
import frFR from './fr-FR'
import esES from './es-ES'
import koKR from './ko-KR'
import arSA from './ar-SA'

const savedLocale = localStorage.getItem('caelab-locale') || 'zh-CN'

// RTL 语言列表
const RTL_LANGUAGES = ['ar-SA']

export function isRTLLanguage(locale: string): boolean {
  return RTL_LANGUAGES.includes(locale)
}

const i18n = createI18n({
  legacy: false,
  locale: savedLocale,
  fallbackLocale: 'zh-CN',
  messages: {
    'zh-CN': zhCN,
    'en-US': enUS,
    'ja-JP': jaJP,
    'de-DE': deDE,
    'fr-FR': frFR,
    'es-ES': esES,
    'ko-KR': koKR,
    'ar-SA': arSA,
  }
})

export default i18n
