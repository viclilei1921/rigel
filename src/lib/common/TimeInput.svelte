<script lang="ts">
  import { tick } from 'svelte';

  import { Completion, logger, TimeToFormat } from '../../utils';

  // 1. 定义 Props
  interface Props {
    priTime?: number;
    label?: string;
    onfocus?: (e: FocusEvent) => void;
    onblur?: (t: number) => void;
    onchange?: (t: number) => void;
    class?: string;
  }

  let {
    priTime = 0,
    label = '',
    onfocus,
    onblur,
    onchange,
    class: className = ''
  }: Props = $props();

  // 2. 响应式状态
  let inputDom = $state<HTMLInputElement | null>(null);

  let time = $state('00:00.00');
  let hour = $state('00');
  let minute = $state('00');
  let second = $state('00');
  let millisecond = $state('000');
  let pos = $state<'hour' | 'minute' | 'second' | 'millisecond'>('second');
  let cache = $state('0');

  /** 获取时间 */
  function getTime() {
    time = `${hour}:${minute}:${second}.${millisecond}`;
  }

  async function changeTime(start?: number, end?: number) {
    getTime();

    await tick();
    if (inputDom && start !== undefined && end !== undefined) {
      inputDom.setSelectionRange(start, end);
    }
  }

  /** 设置光标位置 */
  async function setSelect(
    start: number,
    end: number,
    position: 'hour' | 'minute' | 'second' | 'millisecond' | '' = ''
  ) {
    if (position) {
      pos = position;
    }
    cache = '';
    await tick();
    inputDom?.setSelectionRange(start, end);
  }

  function selectChange(event: Event) {
    if (!inputDom) {
      return;
    }

    const selectionEnd = inputDom.selectionEnd;
    const selectionStart = inputDom.selectionStart;

    if (selectionEnd === null || selectionStart === null) {
      return;
    }

    if (selectionEnd < 3) {
      if (pos === 'hour' && selectionStart === 0) {
        event.preventDefault();
        return;
      }

      const selection = window.getSelection();

      if (selection && selection.rangeCount > 0) {
        selection.removeAllRanges();
      }

      setSelect(0, 2, 'hour');
      return;
    }

    if (selectionEnd < 6) {
      if (pos === 'minute' && selectionStart === 3) {
        event.preventDefault();
        return;
      }

      const selection = window.getSelection();

      if (selection && selection.rangeCount > 0) {
        selection.removeAllRanges();
      }

      setSelect(3, 5, 'minute');
      return;
    }

    if (selectionEnd < 9) {
      if (pos === 'second' && selectionStart === 6) {
        event.preventDefault();
        return;
      }

      const selection = window.getSelection();

      if (selection && selection.rangeCount > 0) {
        selection.removeAllRanges();
      }

      setSelect(6, 8, 'second');
      return;
    }

    if (selectionEnd <= 12) {
      if (pos === 'millisecond' && selectionStart === 9) {
        event.preventDefault();
        return;
      }

      const selection = window.getSelection();

      if (selection && selection.rangeCount > 0) {
        selection.removeAllRanges();
      }

      setSelect(9, 12, 'millisecond');
    }
  }

  function keyNumber(num: string) {
    if (pos === 'millisecond') {
      if (cache.length >= 3) {
        cache = '';
      }
      cache += num;
      millisecond = Completion(Number(cache), 3);
      changeTime(9, 12);
      return;
    }

    if (cache.length > 1 || +cache > 5 || cache === '0') {
      cache = '';
    }

    cache += num;
    const value = Number(cache);
    logger.info(value);

    if (pos === 'hour') {
      hour = Completion(value);
      if (value >= 10) {
        getTime();
        setSelect(3, 5, 'minute');
        return;
      }
      changeTime(0, 2);
      return;
    }

    if (pos === 'minute') {
      minute = Completion(value);
      if (value >= 10) {
        getTime();
        setSelect(6, 8, 'second');
        return;
      }
      changeTime(3, 5);
      return;
    }

    if (pos === 'second') {
      second = Completion(value);
      if (value >= 10) {
        getTime();
        setSelect(9, 12, 'millisecond');
        return;
      }
      changeTime(6, 8);
    }
  }

  function changeOne(direction: number) {
    const selection = window.getSelection();
    let num = 0;

    if (selection) {
      num = Number(selection.toString());
      if (Number.isNaN(num)) {
        num = 0;
      }
    }

    num += direction;

    if (pos === 'hour') {
      if (num < 0 || num > 23) {
        return;
      }
      hour = Completion(num);
      changeTime(0, 2);
      return;
    }

    if (pos === 'minute') {
      if (num < 0 || num > 59) {
        return;
      }
      minute = Completion(num);
      changeTime(3, 5);
      return;
    }

    if (pos === 'second') {
      if (num < 0 || num > 59) {
        return;
      }
      second = Completion(num);
      changeTime(6, 8);
    }

    if (pos === 'millisecond') {
      if (num < 0 || num > 999) {
        return;
      }
      millisecond = Completion(num, 3);
      changeTime(9, 12);
    }
  }

  async function handleFocus(event: FocusEvent) {
    onfocus?.(event);

    await tick();

    const selection = window.getSelection();
    if (selection && selection.rangeCount > 0) {
      selection.removeAllRanges();
    }
  }

  function handleBlur() {
    const t = 60 * 60 * +hour + 60 * +minute + +second + +millisecond / 1000;
    getTime();
    pos = 'millisecond';

    onblur?.(t);
    onchange?.(t);
  }

  function handlePointDown() {
    document.addEventListener('pointerup', handlePointerup);
  }

  function handlePointerup(event: MouseEvent) {
    document.removeEventListener('pointerup', handlePointerup);
    cache = '0';
    selectChange(event);
  }

  function handleKeydown(event: KeyboardEvent) {
    const { code, key } = event;

    if (key === 'Unidentified' || time.length !== 12) {
      changeTime();
      return;
    }

    selectChange(event);

    if (code.includes('Digit') || code.includes('Numpad')) {
      if (code === 'NumpadDecimal') {
        return;
      }

      const num = code.slice(code.length - 1);
      if (Number.isNaN(num)) {
        return;
      }

      keyNumber(num);
      return;
    }

    switch (key) {
      case 'ArrowDown':
        changeOne(-1);
        break;
      case 'ArrowUp':
        changeOne(1);
        break;
      case 'ArrowLeft':
        cache = '';
        if (pos === 'millisecond') {
          setSelect(6, 8, 'second');
          break;
        }
        if (pos === 'second') {
          setSelect(3, 5, 'minute');
          break;
        }
        if (pos === 'minute') {
          setSelect(0, 2, 'hour');
          break;
        }
        if (pos === 'hour') {
          setSelect(9, 12, 'millisecond');
        }
        break;
      case 'ArrowRight':
      case 'Tab':
        cache = '';
        if (pos === 'hour') {
          setSelect(3, 5, 'minute');
          break;
        }
        if (pos === 'minute') {
          setSelect(6, 8, 'second');
          break;
        }
        if (pos === 'second') {
          setSelect(9, 12, 'millisecond');
          break;
        }
        if (pos === 'millisecond') {
          setSelect(0, 2, 'hour');
        }
        break;
      case 'Delete':
      case 'Backspace':
        cache = '';
        if (pos === 'millisecond') {
          millisecond = '000';
          changeTime(9, 12);
          break;
        }
        if (pos === 'second') {
          second = '00';
          changeTime(6, 8);
          break;
        }
        if (pos === 'minute') {
          minute = '00';
          changeTime(3, 5);
          break;
        }
        if (pos === 'hour') {
          hour = '00';
          changeTime(0, 2);
          break;
        }
        break;
    }
  }

  /** 初始化 */
  function initData() {
    const [h, m, s, ms] = TimeToFormat(priTime);

    hour = h;
    minute = m;
    second = s;
    millisecond = ms;

    getTime();
  }

  initData();
</script>

<div class="relative w-full {className}">
  <input
    bind:this={inputDom}
    type="tel"
    value={time}
    onfocus={handleFocus}
    onblur={handleBlur}
    class="peer w-full rounded-md border border-gray-300 bg-transparent p-2 focus:border-blue-600 focus:outline-none"
    placeholder="00:00:00.000"
    onpointerdown={handlePointDown}
    onkeydown={handleKeydown}
    onclick={(e) => e.preventDefault()}
    ondblclick={(e) => e.preventDefault()}
    onpointermove={(e) => {
      e.preventDefault();
      e.stopImmediatePropagation();
    }}
  />

  <div
    class="absolute -top-2 left-2 bg-slate-100 px-1 text-xs text-gray-500 transition-all duration-200 peer-focus:-top-2 peer-focus:text-xs peer-focus:text-blue-600"
  >
    {label}
  </div>
</div>
