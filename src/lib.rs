pub mod yomichan;
// TODO: We might implement our own abstract dictionary entry type.
//       The user could then call a function iterate_dictionary(path_to_dict) -> impl Iterator<Item=DictionaryEntry>.
//       The iterator should return None on error and the user should afterwards be able to
//       check if there were any errors
