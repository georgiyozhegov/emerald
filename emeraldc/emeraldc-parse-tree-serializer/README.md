For now, custom visualizer development is stopped. 

I realized that JSON is much more simpler way to visualize parse tree. Plus, there are lots of existing libraries like *serde* and *serde_json* that help with this kind of problems.

One day, I will definitely implement this feature.

My concept is something like that:

    functuion name(...) ... end

Where `...` represent folded inner nodes.

So, for example, if i click on `...` it expands to something like this:

    function name(...)
        let a = ...
        if ...
            ...
        else
            ...
        end
    end

As in the previous version of this visualizer, it will show a pop-up with detailed info about the current node and its children.

:}
