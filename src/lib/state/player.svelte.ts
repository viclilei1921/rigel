import { getContext, setContext } from "svelte";

class PlayerState {
  private _show = $state(false);
  private _url = $state('');

  get show() {
    return this._show;
  }

  get url() {
    return this._url;
  }

  open(url: string) {
    this._show = true;
    this._url = url;
  }

  close() {
    this._show = false;
    this._url = '';
  }
}

const PLAYER_KEY = Symbol('PLAYER');

export function setPlayerContext() {
    return setContext(PLAYER_KEY, new PlayerState());
}

export function getPlayerContext(): PlayerState {
    return getContext(PLAYER_KEY);
}