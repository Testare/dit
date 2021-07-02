# general
* Readme
* publish to crates.io

# dit_core
* Test code
* Fix BadAction error message and parameters
* Refactor validate
* Actions
  * is_possible function, (takes previous hash and current state): checked before work is started adding it to message list.
  * apply function takes a struct composed of previous message hash key, current message hash key, root hash, and maybe some other data (Maybe all the hashes? Maybe call it "ledger"?). Functions on the struct to generate randomness for actions in a way that can be duplicated from the keys.
    - Using the current work hash  means that it is more or less actually random. The tradeoff is that a player could easily drop that line and repeat it as often as he wants, though that introduces work again each time. Keep in mind that however we use this value, the last X bits of it will be the same each time, so don't just depend on end values for randomness. It can be useful for things you want to be a "keep trying until you succeed" thing, like actions that are repeatable.
    - Using the previous work hash (or one preceding the message X times, or multiple previous hashes) means that the user can't just undo and retry for a different result unless they undo all the way back to that point. However, they COULD just undo and do a different action. We might want to filter free actions, since if we allowed arbitrary marker messages (which we plan to) they could just add markers until they get the result they want, and we're functionally no different from the previous approach.
    - Using the hash of a specific previous action means that the user would have to undo to that action to change the result, which they might not want to do.
    - Using the root hash means the user would have to create a completely new file to get different results. The user is very likely not going to want to go this far, but it only really works for Actions that don't make sense to repeat, since they'll have the same number every time. 
    - Combining these usually don't create a combination of pros and cons.
 * State file headers (Everything before a line marked "---" with the quotes still there so that it is valid JSON)
 * Mode header, first line every time.



# mode_a

* Generate simple maps
* seek out monster
* battle in D&D style
* Andy?

# Andy
* Andy?
