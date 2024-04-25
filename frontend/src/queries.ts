import { gql } from "@apollo/client";

export const GET_CHAINHOOK_STORE = gql`
  query GetChainhookStore {
    chainhookStore {
      store
    }
  }
`;
