<script lang="ts" setup>
import { nextTick, ref } from 'vue'

import { Completion, logger, TimeToFormat } from '../../utils'

type EmitsType = {
  (e: 'focus', event: FocusEvent): void
  (e: 'blur', t: number): void
  (e: 'change', t: number): void
}

const props = withDefaults(defineProps<{
  priTime?: number
}>(), {
  priTime: 0
})

const emits = defineEmits<EmitsType>()

const time = ref('00:00.00')
const hour = ref('00')
const minute = ref('00')
const second = ref('00')
const millisecond = ref('000')
const pos = ref<'hour' | 'minute' | 'second' | 'millisecond'>('second')
const focusing = ref(false)
const cache = ref('0')
const inputDom = ref<HTMLInputElement | null>(null)

/** 获取时间 */
function getTime() {
  time.value = `${hour.value}:${minute.value}:${second.value}.${millisecond.value}`
}

function changeTime(start?: number, end?: number) {
  getTime()
  nextTick(() => {
    if (inputDom.value && start !== undefined && end !== undefined) {
      inputDom.value.setSelectionRange(start, end)
    }
  })
}

/** 设置光标位置 */
function setSelect(start: number, end: number, position: 'hour' | 'minute' | 'second' | 'millisecond' | '' = '') {
  if (position) {
    pos.value = position
  }
  cache.value = ''
  nextTick(() => {
    inputDom.value?.setSelectionRange(start, end)
  })
}

function selectChange(event: Event) {
  if (!inputDom.value) {
    return
  }

  const selectionEnd = inputDom.value.selectionEnd
  const selectionStart = inputDom.value.selectionStart

  if (selectionEnd === null || selectionStart === null) {
    return
  }

  if (selectionEnd < 3) {
    if (pos.value === 'hour' && selectionStart === 0) {
      event.preventDefault()
      return
    }

    const selection = window.getSelection()

    if (selection && selection.rangeCount > 0) {
      selection.removeAllRanges()
    }

    setSelect(0, 2, 'hour')
    return
  }

  if (selectionEnd < 6) {
    if (pos.value === 'minute' && selectionStart === 3) {
      event.preventDefault()
      return
    }

    const selection = window.getSelection()

    if (selection && selection.rangeCount > 0) {
      selection.removeAllRanges()
    }

    setSelect(3, 5, 'minute')
    return
  }

  if (selectionEnd < 9) {
    if (pos.value === 'second' && selectionStart === 6) {
      event.preventDefault()
      return
    }

    const selection = window.getSelection()

    if (selection && selection.rangeCount > 0) {
      selection.removeAllRanges()
    }

    setSelect(6, 8, 'second')
    return
  }

  if (selectionEnd <= 12) {
    if (pos.value === 'millisecond' && selectionStart === 9) {
      event.preventDefault()
      return
    }

    const selection = window.getSelection()

    if (selection && selection.rangeCount > 0) {
      selection.removeAllRanges()
    }

    setSelect(9, 12, 'millisecond')
  }
}

function keyNumber(num: string) {
  if (pos.value === 'millisecond') {
    if (cache.value.length >= 3) {
      cache.value = ''
    }
    cache.value += num
    millisecond.value = Completion(Number(cache.value), 3)
    changeTime(9, 12)
    return
  }

  if (cache.value.length > 1 || +cache.value > 5 || cache.value === '0') {
    cache.value = ''
  }

  cache.value += num
  const value = Number(cache.value)
  logger.info(value)

  if (pos.value === 'hour') {
    hour.value = Completion(value)
    if (value >= 10) {
      getTime()
      setSelect(3, 5, 'minute')
      return
    }
    changeTime(0, 2)
    return
  }

  if (pos.value === 'minute') {
    minute.value = Completion(value)
    if (value >= 10) {
      getTime()
      setSelect(6, 8, 'second')
      return
    }
    changeTime(3, 5)
    return
  }

  if (pos.value === 'second') {
    second.value = Completion(value)
    if (value >= 10) {
      getTime()
      setSelect(9, 12, 'millisecond')
      return
    }
    changeTime(6, 8)
  }
}

function changeOne(direction: number) {
  const selection = window.getSelection()
  let num = 0

  if (selection) {
    num = Number(selection.toString())
    if (Number.isNaN(num)) {
      num = 0
    }
  }

  num += direction

  if (pos.value === 'hour') {
    if (num < 0 || num > 23) {
      return
    }
    hour.value = Completion(num)
    changeTime(0, 2)
    return
  }

  if (pos.value === 'minute') {
    if (num < 0 || num > 59) {
      return
    }
    minute.value = Completion(num)
    changeTime(3, 5)
    return
  }

  if (pos.value === 'second') {
    if (num < 0 || num > 59) {
      return
    }
    second.value = Completion(num)
    changeTime(6, 8)
  }

  if (pos.value === 'millisecond') {
    if (num < 0 || num > 999) {
      return
    }
    millisecond.value = Completion(num, 3)
    changeTime(9, 12)
  }
}

function handleFocus(event: FocusEvent) {
  focusing.value = true
  nextTick(() => {
    const selection = window.getSelection()
    if (selection && selection.rangeCount > 0) {
      selection.removeAllRanges()
    }
  })

  emits('focus', event)
}

function handleBlur() {
  const t = 60 * 60 * +hour.value + 60 * +minute.value + +second.value + +millisecond.value / 1000
  getTime()
  pos.value = 'millisecond'
  focusing.value = false
  emits('blur', t)
  emits('change', t)
}

function handlePointDown() {
  document.addEventListener('pointerup', handlePointerup)
}

function handlePointerup(event: MouseEvent) {
  document.removeEventListener('pointerup', handlePointerup)
  cache.value = '0'
  selectChange(event)
}

function handleKeydown(event: KeyboardEvent) {
  const { code, key } = event

  if (key === 'Unidentified' || time.value.length !== 12) {
    changeTime()
    return
  }

  selectChange(event)

  if (code.includes('Digit') || code.includes('Numpad')) {
    if (code === 'NumpadDecimal') {
      return
    }

    const num = code.slice(code.length - 1)
    if (Number.isNaN(num)) {
      return
    }

    keyNumber(num)
    return
  }

  switch (key) {
    case 'ArrowDown':
      changeOne(-1)
      break
    case 'ArrowUp':
      changeOne(1)
      break
    case 'ArrowLeft':
      cache.value = ''
      if (pos.value === 'millisecond') {
        setSelect(6, 8, 'second')
        break
      }
      if (pos.value === 'second') {
        setSelect(3, 5, 'minute')
        break
      }
      if (pos.value === 'minute') {
        setSelect(0, 2, 'hour')
        break
      }
      if (pos.value === 'hour') {
        setSelect(9, 12, 'millisecond')
      }
      break
    case 'ArrowRight':
    case 'Tab':
      cache.value = ''
      if (pos.value === 'hour') {
        setSelect(3, 5, 'minute')
        break
      }
      if (pos.value === 'minute') {
        setSelect(6, 8, 'second')
        break
      }
      if (pos.value === 'second') {
        setSelect(9, 12, 'millisecond')
        break
      }
      if (pos.value === 'millisecond') {
        setSelect(0, 2, 'hour')
      }
      break
    case 'Delete':
    case 'Backspace':
      cache.value = ''
      if (pos.value === 'millisecond') {
        millisecond.value = '000'
        changeTime(9, 12)
        break
      }
      if (pos.value === 'second') {
        second.value = '00'
        changeTime(6, 8)
        break
      }
      if (pos.value === 'minute') {
        minute.value = '00'
        changeTime(3, 5)
        break
      }
      if (pos.value === 'hour') {
        hour.value = '00'
        changeTime(0, 2)
        break
      }
      break
  }
}

/** 初始化 */
function initData() {
  const [h, m, s, ms] = TimeToFormat(props.priTime)

  hour.value = h
  minute.value = m
  second.value = s
  millisecond.value = ms

  getTime()
}

initData()
</script>

<template>
  <v-text-field
    ref="inputDom" v-model="time" type="tel" @focus="handleFocus" @blur="handleBlur"
    @keydown.stop.prevent="handleKeydown" @pointerdown="handlePointDown" @click.prevent @dblclick.prevent
    @pointermove.prevent.stop
  />
</template>
