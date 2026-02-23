import { useCallback, useEffect } from 'react';
import { useAppState, useAppDispatch } from '../stores/appStore';
import * as api from '../lib/api';

export function useRuleset() {
  const state = useAppState();
  const dispatch = useAppDispatch();

  const loadRulesets = useCallback(async () => {
    try {
      const rulesets = await api.listRulesets();
      dispatch({ type: 'SET_RULESETS', rulesets });
    } catch (e) {
      dispatch({ type: 'SET_ERROR', error: String(e) });
    }
  }, [dispatch]);

  const createRuleset = useCallback(
    async (name: string, content: string) => {
      try {
        const ruleset = await api.createRuleset({ name, content });
        dispatch({ type: 'ADD_RULESET', ruleset });
        return ruleset;
      } catch (e) {
        dispatch({ type: 'SET_ERROR', error: String(e) });
        throw e;
      }
    },
    [dispatch],
  );

  const updateRuleset = useCallback(
    async (id: string, name: string, content: string) => {
      try {
        const ruleset = await api.updateRuleset({ id, name, content });
        dispatch({ type: 'UPDATE_RULESET', ruleset });
        return ruleset;
      } catch (e) {
        dispatch({ type: 'SET_ERROR', error: String(e) });
        throw e;
      }
    },
    [dispatch],
  );

  const deleteRuleset = useCallback(
    async (id: string) => {
      try {
        await api.deleteRuleset(id);
        dispatch({ type: 'DELETE_RULESET', rulesetId: id });
      } catch (e) {
        dispatch({ type: 'SET_ERROR', error: String(e) });
        throw e;
      }
    },
    [dispatch],
  );

  useEffect(() => {
    loadRulesets();
  }, [loadRulesets]);

  return {
    rulesets: state.rulesets,
    loadRulesets,
    createRuleset,
    updateRuleset,
    deleteRuleset,
  };
}
