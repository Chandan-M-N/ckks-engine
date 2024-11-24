searchState.loadedDescShard("bitvec", 0, "<code>bitvec</code> — Addressable Bits\nMemory access guards.\nA statically-allocated, fixed-size, buffer containing a …\nConstructs a new <code>BitArray</code> from a bit-pattern description.\nCreates a borrowed <code>BitSlice</code> in the local scope.\nRepresentations of the <code>BitSlice</code> region memory model.\nBatched load/store access to bitfields.\nWell-typed counters and register descriptors.\nConstructor macros for the crate’s collection types.\nMemory element descriptions.\nOrdering of bits within register elements.\n<code>bitvec</code> symbol export.\nMirror of the <code>core::ptr</code> module and <code>bitvec</code>-specific pointer …\nA dynamically-sized view into individual bits of a memory …\nMemory modeling.\n<code>BitSlice</code> view adapters for memory regions.\nAbstracts over the instructions used when accessing a …\nRestricts memory modification to only exclusive references.\nA wrapper over a shared-mutable type that forbids writing …\nA wrapper over a shared-mutable type that forbids writing …\nA wrapper over a shared-mutable type that forbids writing …\nA wrapper over a shared-mutable type that forbids writing …\nA wrapper over a shared-mutable type that forbids writing …\nThe register type being guarded against shared mutation.\nThe accessor type being prevented from mutating while …\nClears any number of bits in a memory register to <code>0</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGets the function that writes <code>value</code> into all bits under a …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nInverts any number of bits in a memory register.\nReads the value out of memory only if a shared reference …\nSets any number of bits in a memory register to <code>1</code>.\nWrites a value into memory only if an exclusive reference …\nWrites a value to one bit in a memory register.\nAn array of individual bits, able to be held by value on …\nA by-value bit-array iterator.\nViews the array as a <code>BitSlice</code>.\nReturns an immutable slice of all bits that have not been …\nViews the interior buffer.\nViews the array as a mutable <code>BitSlice</code>.\nReturns a mutable slice of all bits that have not been …\nMutably views the interior buffer.\nViews the array as a mutable slice of its underlying …\nViews the array as a slice of its underlying memory …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nWraps a buffer in a <code>BitArray</code>.\nRemoves the <code>BitArray</code> wrapper, leaving the contained buffer.\nConstructs a new <code>BitArray</code> with its memory set to zero.\nGranular representation of the memory region containing a …\nGranular representation of the memory region containing a …\nGranular representation of the memory region containing a …\nGranular representation of the memory region containing a …\nIndicates that a <code>BitSlice</code> is contained entirely in the …\nIndicates that a <code>BitSlice</code> is contained entirely in the …\nIndicates that a <code>BitSlice</code> is contained entirely in the …\nIndicates that a <code>BitSlice</code> is contained entirely in the …\nIndicates that a <code>BitSlice</code> region touches at least one edge …\nIndicates that a <code>BitSlice</code> region touches at least one edge …\nIndicates that a <code>BitSlice</code> region touches at least one edge …\nIndicates that a <code>BitSlice</code> region touches at least one edge …\nAttempts to view the domain as an enclave variant.\nAttempts to view the domain as an enclave variant.\nAttempts to view the domain as an enclave variant.\nAttempts to view the domain as an enclave variant.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nAttempts to view the domain as a region variant.\nAttempts to view the domain as a region variant.\nAttempts to view the domain as the region variant.\nAttempts to view the domain as the region variant.\nThe original <code>BitSlice</code> used to create this bit-domain view.\nAny bits inside elements that the source <code>BitSlice</code> …\nThe start index of the <code>BitSlice</code>.\nAny bits that partially-fill the base element of the slice …\nThe end index of the <code>BitSlice</code>.\nAny bits that partially fill the last element of the slice …\nThe original <code>BitSlice</code> used to create this bit-domain view.\nAny bits inside elements that the source <code>BitSlice</code> …\nThe start index of the <code>BitSlice</code>.\nAny bits that partially-fill the base element of the slice …\nThe end index of the <code>BitSlice</code>.\nAny bits that partially fill the last element of the slice …\nAll fully-spanned, unaliased, elements.\nAn aliased view of the element containing the <code>BitSlice</code>.\nThe start index of the <code>BitSlice</code>.\nIf the <code>BitSlice</code> started in the interior of its first …\nThe end index of the <code>BitSlice</code>.\nIf the <code>BitSlice</code> ended in the interior of its last element, …\nAll fully-spanned, unaliased, elements.\nAn aliased view of the element containing the <code>BitSlice</code>.\nThe start index of the <code>BitSlice</code>.\nIf the <code>BitSlice</code> started in the interior of its first …\nThe end index of the <code>BitSlice</code>.\nIf the <code>BitSlice</code> ended in the interior of its last element, …\nPerforms C-style bitfield access through a <code>BitSlice</code>.\nLoads the bits in the <code>self</code> region into a local value.\nLoads from <code>self</code>, using big-endian element <code>T</code> ordering.\nLoads from <code>self</code>, using little-endian element <code>T</code> ordering.\nStores a sequence of bits from the user into the domain of …\nStores into <code>self</code>, using big-endian element ordering.\nStores into <code>self</code>, using little-endian element ordering.\nA full mask.\nA semantic index counter within a register element <code>R</code>.\nMarks an index that is invalid for a register type.\nA multi-bit selection mask for a register <code>R</code>.\nAn electrical position counter within a register element <code>R</code>.\nA one-hot selection mask for a register element <code>R</code>.\nA semantic index counter within <em>or one bit past the end of</em> …\nThe inclusive maximum index within an element <code>R</code>.\nThe inclusive maximum tail within an element <code>R</code>.\nThe inclusive minimum index within an element <code>R</code>.\nThe inclusive minimum tail within an element <code>R</code>.\nAn empty mask.\nCasts to a new index type.\nCreates a new mask with a selector bit activated.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nInserts a selector bit into an existing mask.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nComputes the bit selector for <code>self</code> as an accessor mask.\nComputes the bit selector for <code>self</code> as an accessor mask.\nComputes a bit-mask for <code>self</code>. This is a type-cast.\nWraps a counter value as a known-good index into an <code>R</code> …\nWraps a counter value as a known-good tail of an <code>R</code> …\nWraps a counter value as a known-good position within an <code>R</code> …\nWraps a counter value as a known-good selection of an <code>R</code> …\nWraps any <code>R</code> value as a bit-mask.\nWraps a counter value as an assumed-good index into an <code>R</code> …\nWraps a counter value as an assumed-good position within …\nWraps a counter value as an assumed-good selection of an <code>R</code> …\nIncrements an index counter, wrapping at the back edge of …\nComputes the jump distance for some number of bits away …\nComputes the bit position corresponding to <code>self</code> under some …\nDecrements an index counter, wrapping at the front edge of …\nIterates over all indices between an inclusive start and …\nIterates over all possible index values.\nIterates over all possible selector values.\nIterates over all tail indices at and after an inclusive …\nComputes the bit selector corresponding to <code>self</code> under an …\nComputes the bit selector corresponding to <code>self</code>.\nComputes the span information for a region beginning at …\nTests whether the mask contains a given selector bit.\nRemoves the index wrapper, leaving the internal counter.\nRemoves the error wrapper, leaving the internal counter.\nRemoves the tail wrapper, leaving the internal counter.\nRemoves the position wrapper, leaving the internal counter.\nRemoves the selector wrapper, leaving the internal counter.\nRemoves the mask wrapper, leaving the internal value.\nThe literal <code>!0</code>.\nThe bit width of the integer.\nDescription of an integer memory element.\nDescription of a processor register.\nThe number of bits required to store an index in the range …\nA mask over all bits that can be used as an index within …\nThe literal <code>1</code>.\nAn ordering over a register.\nA default bit ordering.\nTraverses a register from the least significant bit to the …\nTraverses a register from the most significant bit to the …\nConverts a semantic bit index into an electrical bit …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConstructs a multiple-bit selector mask for batched …\nConverts a semantic bit index into a one-hot selector mask.\nVerifies a <code>BitOrder</code> implementation’s adherence to the …\nVerifies a <code>BitOrder</code> implementation’s adherence to the …\nConstructs a new <code>BitArray</code> from a bit-pattern description.\nCreates a borrowed <code>BitSlice</code> in the local scope.\nA non-null, well-aligned, <code>BitStore</code> element address.\nAn error produced when consuming <code>BitStore</code> memory addresses.\nPointer to an individual bit in a memory element. …\nErrors produced by invalid bit-pointer components.\nEquivalent to <code>Range&lt;BitPtr&lt;M, O, T&gt;&gt;</code>.\nA proxy reference, equivalent to C++ …\nAn error produced when creating <code>BitSpan</code> encoded references.\nAn immutable pointer.\nThe dangling pointer. This selects the starting bit of the …\nThe canonical empty range. All ranges with zero length are …\nThe element address was somehow invalid.\nThe base <code>BitPtr</code> is invalid.\nThe bit index was somehow invalid.\n<code>Address</code> cannot be misaligned for the referent type <code>T</code>.\nA mutable pointer. Contexts with a <code>Mutable</code> may lower to …\n<code>Address</code> cannot use the null pointer.\n<code>BitSpan</code> domains have an address ceiling.\n<code>BitSpan</code> domains have a length ceiling.\nCalculates the offset from a pointer (convenience for …\nComputes the offset (in bits) that needs to be applied to …\nProduces a proxy mutable reference to the referent bit.\nProduces a proxy reference to the referent bit.\nAdds write permissions to a bit-pointer.\nForms a raw bit-slice from a bit-pointer and a length.\nPerforms the same functionality as <code>bitslice_from_raw_parts</code>…\nCasts to a bit-pointer of another storage type, preserving …\nReturns <code>true</code> if the <code>pointer</code> is contained in the range.\nCopies <code>count</code> bits from <code>src</code> to <code>dst</code>. The source and …\nCopies <code>count</code> bits from <code>src</code> to <code>self</code>. The source and …\nCopies <code>count</code> bits from <code>src</code> to <code>self</code>. The source and …\nCopies <code>count</code> bits from <code>src</code> to <code>dst</code>. The source and …\nCopies <code>count</code> bits from <code>self</code> to <code>dest</code>. The source and …\nCopies <code>count</code> bits from <code>self</code> to <code>dest</code>. The source and …\nThe higher bound of the range (exclusive).\nCompares raw bit-pointers for equality.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConverts a bit-pointer into a proxy bit-reference.\nConstructs a <code>BitPtr</code> from an element reference.\nAttempts to construct a <code>BitPtr</code> from an element location.\nConstructs a <code>BitPtr</code> from a slice reference.\nAttempts to construct a <code>BitPtr</code> from an element location.\nConstructs a <code>BitPtr</code> from an element reference.\nConstructs a <code>BitPtr</code> from a slice reference.\nHash a raw bit-pointer.\nRemoves write permissions from a bit-pointer.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nDecays the bit-reference to an ordinary bit-pointer.\nConverts a bit-pointer into a proxy bit-reference.\nConverts the structure into an actual <code>Range</code>. The <code>Range</code> …\nTests if the range is empty (the distance between pointers …\nTests if a bit-pointer is the null value.\nConstructs a <code>BitPtr</code> from a memory location and a bit index.\nCalculates the offset from a pointer.\nCalculates the distance between two pointers. The returned …\nGets the pointer to the base memory location containing …\nGets the pointer to the base memory location containing …\nProduces a pointer range starting at <code>self</code> and running for …\nDestructures the range back into its start and end …\nDecomposes the pointer into its element address and bit …\nReads the bit from <code>src</code>.\nReads the bit from <code>*self</code>.\nPerforms a volatile read of the bit from <code>src</code>.\nPerforms a volatile read of the bit from <code>self</code>.\nMoves <code>src</code> into the pointed <code>dst</code>, returning the previous <code>dst</code> …\nMoves <code>src</code> into the referenced bit, returning the previous …\nReplaces the bit at <code>*self</code> with <code>src</code>, returning the old bit.\nWrites a bit into the proxied location without an …\nThe lower bound of the range (inclusive).\nCalculates the offset from a pointer (convenience for …\nSwaps the values at two mutable locations.\nSwaps the values at two mutable locations, without …\nSwaps the bits at two mutable locations. They may overlap.\nSwaps <code>count</code> bits between the two regions of memory …\nGets the address as a read-only pointer.\nGets the address as a write-capable pointer.\nTries to construct a <code>BitPtr</code> from a memory location and a …\nCalculates the offset from a pointer using wrapping …\nCalculates the offset from a pointer using wrapping …\nCalculates the offset from a pointer using wrapping …\nOverwrites a memory location with the given bit.\nOverwrites a memory location with the given bit.\nPerforms a volatile write of a memory location with the …\nPerforms a volatile write of a memory location with the …\nA slice of individual bits, anywhere in memory.\nA helper trait used for indexing operations.\nAn iterator over a <code>BitSlice</code> in (non-overlapping) chunks (…\nAn iterator over a <code>BitSlice</code> in (non-overlapping) chunks (…\nAn iterator over a <code>BitSlice</code> in (non-overlapping) mutable …\nAn iterator over a <code>BitSlice</code> in (non-overlapping) mutable …\nThe output type for immutable accessors.\nImmutable <code>BitSlice</code> iterator.\nMutable <code>BitSlice</code> iterator.\nEnumerates bits in a <code>BitSlice</code> that are set to <code>1</code>.\nEnumerates bits in a <code>BitSlice</code> that are cleared to <code>0</code>.\nThe inclusive maximum length of a <code>BitSlice&lt;_, T&gt;</code>.\nThe inclusive maximum length that a slice <code>[T]</code> can be for …\nThe output type for mutable accessors.\nAn iterator over a <code>BitSlice</code> in (non-overlapping) chunks (…\nAn iterator over a <code>BitSlice</code> in (non-overlapping) chunks (…\nAn iterator over a <code>BitSlice</code> in (non-overlapping) mutable …\nAn iterator over a <code>BitSlice</code> in (non-overlapping) mutable …\nAn iterator over subslices separated by bits that match a …\nAn iterator over subslices separated by bits that match a …\nAn iterator over subslices separated by bits that match a …\nAn iterator over subslices separated by bits that match a …\nAn iterator over subslices separated by bits that match a …\nAn iterator over the mutable subslices which are separated …\nAn iterator over subslices separated by bits that match a …\nAn iterator over subslices separated by bits that match a …\nAn iterator over overlapping subslices of length <code>size</code>.\nTransmute the bit-slice to a bit-slice of another type, …\nTransmute the bit-slice to a bit-slice of another type, …\nTests if <em>all</em> bits in the slice domain are set (logical <code>∧</code>…\nTests if <em>any</em> bit in the slice is set (logical <code>∨</code>).\nReturns a raw bit-pointer to the base of the bit-slice’s …\nReturns the two raw bit-pointers spanning the bit-slice.\nViews the underlying data as a subslice of the original …\nReturns an unsafe mutable bit-pointer to the bit-slice’s …\nReturns the two unsafe mutable bit-pointers spanning the …\nViews the underlying memory containing the slice.\nSplits the slice into subslices at alias boundaries.\nSplits the slice into subslices at alias boundaries.\nAdapts the iterator to yield <code>&amp;bool</code> references rather than …\nAdapts the iterator to yield <code>bool</code> values rather than <code>BitRef</code>…\nReturns an iterator over <code>chunk_size</code> bits of the slice at a …\nReturns an iterator over <code>chunk_size</code> bits of the slice at a …\nReturns an iterator over <code>chunk_size</code> bits of the slice at a …\nReturns an iterator over <code>chunk_size</code> bits of the slice at a …\nCopies the bits from <code>src</code> into <code>self</code>.\nReturns <code>true</code> if the slice contains a subslice that matches …\nForwards to <code>by_val</code>.\nCopies all bits from <code>src</code> into <code>self</code>, using a memcpy wherever\nCopies bits from one part of the slice to another part of …\nCopies bits from one part of the slice to another part of …\nCounts the number of bits set to <code>1</code> in the slice contents.\nCounts the number of bits cleared to <code>0</code> in the slice …\nViews the underlying memory containing the slice, split at …\nViews the underlying memory containing the slice, split at …\nProduces the empty slice reference.\nProduces the empty mutable slice reference.\nReturns <code>true</code> if <code>needle</code> is a suffix of the slice.\nReturns the first bit of the slice, or <code>None</code> if it is empty.\nReturns a mutable pointer to the first bit of the slice, …\nGets the index of the first bit in the bit-slice set to <code>1</code>.\nGets the index of the first bit in the bit-slice set to <code>0</code>.\nApplies a function to each bit in the slice.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstructs a shared <code>&amp;BitSlice</code> reference over a shared …\nConstructs an exclusive <code>&amp;mut BitSlice</code> reference over an …\nConverts a reference to <code>T</code> into a <code>BitSlice</code> over one element.\nForms a bit-slice from a bit-pointer and a length.\nPerforms the same functionality as <code>from_raw_parts</code>, except …\nPerforms the same functionality as <code>from_raw_parts</code>, without …\nPerforms the same functionality as <code>from_raw_parts_mut</code>, …\nConverts a reference to <code>T</code> into a <code>BitSlice</code> over one element.\nConstructs a shared <code>&amp;BitSlice</code> reference over a slice.\nConstructs an exclusive <code>&amp;mut BitSlice</code> reference over a …\nConverts a slice reference into a <code>BitSlice</code> reference …\nConverts a slice reference into a <code>BitSlice</code> reference …\nReturns a shared reference to the output at this location, …\nReturns a reference to a bit or subslice depending on the …\nReturns a mutable reference to the output at this …\nReturns a mutable reference to a bit or subslice depending …\nReturns a shared reference to the output at this location, …\nReturns a reference to a bit or subslice, without doing …\nReturns a mutable reference to the output at this …\nReturns a mutable reference to a bit or subslice, without …\nReturns a shared reference to the output at this location, …\nLooks up a single bit by semantic index.\nReturns a mutable reference to the output at this …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nViews the underlying data as a subslice of the original …\nReturns the remainder of the original <code>BitSlice</code> that is not …\nReturns the remainder of the original <code>BitSlice</code> that is not …\nReturns <code>true</code> if the slice has a length of 0.\nReturns an iterator over the slice.\nReturns an iterator that allows modifying each bit.\nEnumerates all bits in a <code>BitSlice</code> that are set to <code>1</code>.\nEnumerates all bits in a <code>BitSlice</code> that are cleared to <code>0</code>.\nReturns the last bit of the slice, or <code>None</code> if it is empty.\nReturns a mutable pointer to the last bit in the slice.\nGets the index of the last bit in the bit-slice set to <code>1</code>.\nGets the index of the last bit in the bit-slice set to <code>0</code>.\nCounts the number of bits from the start of the bit-slice …\nCounts the number of bits from the start of the bit-slice …\nReturns the number of bits in the slice.\nLoads from <code>self</code>, using big-endian element ordering if <code>self</code> …\nLoads from <code>self</code>, using big-endian element ordering if <code>self</code> …\nLoads from <code>self</code>, using little-endian element ordering if …\nLoads from <code>self</code>, using little-endian element ordering if …\nTests if <em>any</em> bit in the slice is unset (logical <code>¬∧</code>).\nTests if <em>all</em> bits in the slice are unset (logical <code>¬∨</code>).\nProduces the absolute offset in bits between two slice …\nReturns an iterator over <code>chunk_size</code> bits of the slice at a …\nReturns an iterator over <code>chunk_size</code> bits of the slice at a …\nReturns an iterator over <code>chunk_size</code> bits of the slice at a …\nReturns an iterator over <code>chunk_size</code> bits of the slice at a …\nReturns the remainder of the original <code>BitSlice</code> that is not …\nReturns the remainder of the original <code>BitSlice</code> that is not …\nAdapts the iterator to no longer mark its yielded items as …\nAdapts the iterator to no longer mark its yielded items as …\nAdapts the iterator to no longer mark its yielded items as …\nAdapts the iterator to no longer mark its yielded items as …\nAdapts the iterator to no longer mark its yielded items as …\nAdapts the iterator to no longer mark its yielded items as …\nAdapts the iterator to no longer mark its yielded items as …\nAdapts the iterator to no longer mark its yielded items as …\nAdapts the iterator to no longer mark its yielded items as …\nReverses the order of bits in the slice, in place.\nRotates the slice in-place such that the first <code>by</code> bits of …\nRotates the slice in-place such that the first …\nReturns an iterator over subslices separated by bits that …\nReturns an iterator over mutable subslices separated by …\nReturns an iterator over subslices separated by bits that …\nReturns an iterator over subslices separated by bits that …\nWrites a new bit at a given index.\nWrites a new bit at a given index.\nWrites a new bit at a given index, without doing bounds …\nSets all bits in the slice to a value.\nWrites a new bit at a given index, without doing bounds …\nShifts the contents of a bit-slice left (towards index <code>0</code>).\nShifts the contents of a bit-slice right (towards index …\nTests whether the slice has some, but not all, bits set …\nReturns an iterator over subslices separated by bits that …\nDivides one slice into two at an index.\nSplits a mutable slice at some mid-point.\nDivides one mutable slice into two at an index.\nDivides one slice into two at an index, without performing …\nDivides one mutable slice into two at an index.\nReturns the first and all the rest of the bits of the …\nReturns the first and all the rest of the bits of the …\nReturns the last and all the rest of the bits of the …\nReturns the last and all the rest of the bits of the …\nReturns an iterator over mutable subslices separated by …\nReturns an iterator over subslices separated by bits that …\nReturns an iterator over subslices separated by bits that …\nReturns <code>true</code> if <code>needle</code> is a prefix of the slice.\nStores into <code>self</code>, using big-endian element ordering if <code>self</code>…\nStores into <code>self</code>, using big-endian element ordering if <code>self</code>…\nStores into <code>self</code>, using little-endian element ordering if …\nStores into <code>self</code>, using little-endian element ordering if …\nSwaps two bits in the slice.\nSwaps two bits in the slice.\nSwaps all bits in <code>self</code> with those in <code>other</code>.\nCounts the number of bits from the end of the bit-slice to …\nCounts the number of bits from the end of the bit-slice to …\nReturns an iterator over all contiguous windows of length …\nA type that selects appropriate load/store instructions …\nA sibling <code>BitStore</code> implementor. It is used when a <code>BitSlice</code> …\nCommon interface for memory regions.\nThe register type used in the slice region underlying a …\nThe inverse of <code>Alias</code>. It is used when a <code>BitSlice</code> removes …\nReads a single bit out of the memory system according to …\nLoads a value out of the memory system according to the …\nStores a value into the memory system according to the …\nViews a region as an immutable <code>BitSlice</code> only.\nViews a region as a mutable <code>BitSlice</code>.\nCreates a <code>BitSlice</code> view over some type that supports it.\nThe region’s storage type.\nViews memory as a slice of immutable bits.\nViews memory as a slice of mutable bits.\nViews a memory region as a <code>BitSlice</code>.\nViews a memory region as a mutable <code>BitSlice</code>.")